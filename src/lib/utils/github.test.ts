import {
  afterEach,
  beforeAll,
  describe,
  expect,
  it,
  vi,
  type Mock,
} from "vitest";
import { arch, platform } from "@tauri-apps/plugin-os";
import { listOfficialReleases } from "./github";
import { init } from "svelte-i18n";
import { initLocales } from "$lib/i18n/i18n";

vi.mock("@tauri-apps/plugin-os");

global.fetch = vi.fn();

function createFetchResponse(data: any) {
  return {
    ok: true,
    status: 200,
    headers: new Map(),
    json: () => new Promise((resolve) => resolve(data)),
  };
}

function createFakeGithubReleaseAsset(assetName: string) {
  return {
    url: "https://api.github.com/repos/open-goal/jak-project/releases/assets/115111791",
    id: 115111791,
    node_id: "RA_kwDOEUK6OM4G3Hdv",
    name: assetName,
    label: "",
    uploader: {
      login: "github-actions[bot]",
      id: 41898282,
      node_id: "MDM6Qm90NDE4OTgyODI=",
      avatar_url: "https://avatars.githubusercontent.com/in/15368?v=4",
      gravatar_id: "",
      url: "https://api.github.com/users/github-actions%5Bbot%5D",
      html_url: "https://github.com/apps/github-actions",
      followers_url:
        "https://api.github.com/users/github-actions%5Bbot%5D/followers",
      following_url:
        "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
      gists_url:
        "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
      starred_url:
        "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
      subscriptions_url:
        "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
      organizations_url:
        "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
      repos_url: "https://api.github.com/users/github-actions%5Bbot%5D/repos",
      events_url:
        "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
      received_events_url:
        "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
      type: "Bot",
      site_admin: false,
    },
    content_type: "application/x-gtar",
    state: "uploaded",
    size: 21698082,
    download_count: 399,
    created_at: "2023-07-01T06:38:26Z",
    updated_at: "2023-07-01T06:38:28Z",
    browser_download_url: `https://github.com/open-goal/jak-project/releases/download/v0.1.38/${assetName}`,
  };
}

function createFakeGithubRelease(assetNames: string[]) {
  const assets = [];
  for (const assetName of assetNames) {
    assets.push(createFakeGithubReleaseAsset(assetName));
  }
  return {
    url: "https://api.github.com/repos/open-goal/jak-project/releases/110648537",
    assets_url:
      "https://api.github.com/repos/open-goal/jak-project/releases/110648537/assets",
    upload_url:
      "https://uploads.github.com/repos/open-goal/jak-project/releases/110648537/assets{?name,label}",
    html_url: "https://github.com/open-goal/jak-project/releases/tag/v0.1.38",
    id: 110648537,
    author: {
      login: "OpenGOALBot",
      id: 99294829,
      node_id: "U_kgDOBesebQ",
      avatar_url: "https://avatars.githubusercontent.com/u/99294829?v=4",
      gravatar_id: "",
      url: "https://api.github.com/users/OpenGOALBot",
      html_url: "https://github.com/OpenGOALBot",
      followers_url: "https://api.github.com/users/OpenGOALBot/followers",
      following_url:
        "https://api.github.com/users/OpenGOALBot/following{/other_user}",
      gists_url: "https://api.github.com/users/OpenGOALBot/gists{/gist_id}",
      starred_url:
        "https://api.github.com/users/OpenGOALBot/starred{/owner}{/repo}",
      subscriptions_url:
        "https://api.github.com/users/OpenGOALBot/subscriptions",
      organizations_url: "https://api.github.com/users/OpenGOALBot/orgs",
      repos_url: "https://api.github.com/users/OpenGOALBot/repos",
      events_url: "https://api.github.com/users/OpenGOALBot/events{/privacy}",
      received_events_url:
        "https://api.github.com/users/OpenGOALBot/received_events",
      type: "User",
      site_admin: false,
    },
    node_id: "RE_kwDOEUK6OM4GmFzZ",
    tag_name: "v0.1.38",
    target_commitish: "master",
    name: "v0.1.38",
    draft: false,
    prerelease: false,
    created_at: "2023-07-01T06:09:09Z",
    published_at: "2023-07-01T06:38:29Z",
    assets: assets,
    tarball_url:
      "https://api.github.com/repos/open-goal/jak-project/tarball/v0.1.38",
    zipball_url:
      "https://api.github.com/repos/open-goal/jak-project/zipball/v0.1.38",
    body: "## What's Changed\n* ci: ensure linux runners have the proper OpenGL headers by @xTVaser in https://github.com/open-goal/jak-project/pull/2790\n\n\n**Full Changelog**: https://github.com/open-goal/jak-project/compare/v0.1.37...v0.1.38",
    reactions: {
      url: "https://api.github.com/repos/open-goal/jak-project/releases/110648537/reactions",
      total_count: 4,
      "+1": 0,
      "-1": 0,
      laugh: 0,
      hooray: 3,
      confused: 0,
      heart: 0,
      rocket: 0,
      eyes: 1,
    },
    mentions_count: 1,
  };
}

beforeAll(async () => {
  await initLocales(true);
});

describe("listOfficialReleases", () => {
  afterEach(() => {
    vi.clearAllMocks();
    vi.resetAllMocks();
  });

  it("should retrieve intel macOS releases properly", async () => {
    vi.mocked(platform).mockImplementation(() => "macos");
    vi.mocked(arch).mockImplementation(() => "x86_64");
    (fetch as Mock).mockResolvedValue(
      createFetchResponse([
        createFakeGithubRelease([
          "opengoal-macos-intel-v0.0.1.tar.gz",
          "opengoal-windows-v0.0.1.zip",
          "opengoal-linux-v0.0.1.tar.gz",
        ]),
      ]),
    );
    const releases = await listOfficialReleases();
    expect(releases.length).toBe(1);
    expect(
      releases[0].downloadUrl?.endsWith("opengoal-macos-intel-v0.0.1.tar.gz"),
    ).toBeTruthy();
  });

  it("should not retrieve macOS ARM releases", async () => {
    vi.mocked(platform).mockImplementation(() => "macos");
    vi.mocked(arch).mockImplementation(() => "arm");
    (fetch as Mock).mockResolvedValue(
      createFetchResponse([
        createFakeGithubRelease([
          "opengoal-macos-intel-v0.0.1.tar.gz",
          "opengoal-windows-v0.0.1.zip",
          "opengoal-linux-v0.0.1.tar.gz",
        ]),
      ]),
    );
    const releases = await listOfficialReleases();
    expect(releases.length).toBe(1);
    expect(releases[0].downloadUrl).toBeUndefined();
  });

  it("should retrieve windows releases properly", async () => {
    vi.mocked(platform).mockImplementation(() => "windows");
    vi.mocked(arch).mockImplementation(() => "x86_64");
    (fetch as Mock).mockResolvedValue(
      createFetchResponse([
        createFakeGithubRelease([
          "opengoal-macos-intel-v0.0.1.tar.gz",
          "opengoal-windows-v0.0.1.zip",
          "opengoal-linux-v0.0.1.tar.gz",
        ]),
      ]),
    );
    const releases = await listOfficialReleases();
    expect(releases.length).toBe(1);
    expect(
      releases[0].downloadUrl?.endsWith("opengoal-windows-v0.0.1.zip"),
    ).toBeTruthy();
  });

  it("should retrieve linux releases properly", async () => {
    vi.mocked(platform).mockImplementation(() => "linux");
    vi.mocked(arch).mockImplementation(() => "x86_64");
    (fetch as Mock).mockResolvedValue(
      createFetchResponse([
        createFakeGithubRelease([
          "opengoal-macos-intel-v0.0.1.tar.gz",
          "opengoal-windows-v0.0.1.zip",
          "opengoal-linux-v0.0.1.tar.gz",
        ]),
      ]),
    );
    const releases = await listOfficialReleases();
    expect(releases.length).toBe(1);
    expect(
      releases[0].downloadUrl?.endsWith("opengoal-linux-v0.0.1.tar.gz"),
    ).toBeTruthy();
  });
});

describe("getLatestOfficialRelease", () => {
  afterEach(() => {
    vi.clearAllMocks();
    vi.resetAllMocks();
  });

  it("should retrieve intel macOS releases properly", async () => {
    vi.mocked(platform).mockImplementation(() => "macos");
    vi.mocked(arch).mockImplementation(() => "x86_64");
    (fetch as Mock).mockResolvedValue(
      createFetchResponse([
        createFakeGithubRelease([
          "opengoal-macos-intel-v0.0.1.tar.gz",
          "opengoal-windows-v0.0.1.zip",
          "opengoal-linux-v0.0.1.tar.gz",
        ]),
      ]),
    );
    const releases = await listOfficialReleases();
    expect(releases.length).toBe(1);
    expect(
      releases[0].downloadUrl?.endsWith("opengoal-macos-intel-v0.0.1.tar.gz"),
    ).toBeTruthy();
  });

  it("should not retrieve macOS ARM releases", async () => {
    vi.mocked(platform).mockImplementation(() => "macos");
    vi.mocked(arch).mockImplementation(() => "arm");
    (fetch as Mock).mockResolvedValue(
      createFetchResponse([
        createFakeGithubRelease([
          "opengoal-macos-intel-v0.0.1.tar.gz",
          "opengoal-windows-v0.0.1.zip",
          "opengoal-linux-v0.0.1.tar.gz",
        ]),
      ]),
    );
    const releases = await listOfficialReleases();
    expect(releases.length).toBe(1);
    expect(releases[0].downloadUrl).toBeUndefined();
  });

  it("should retrieve windows releases properly", async () => {
    vi.mocked(platform).mockImplementation(() => "windows");
    vi.mocked(arch).mockImplementation(() => "x86_64");
    (fetch as Mock).mockResolvedValue(
      createFetchResponse([
        createFakeGithubRelease([
          "opengoal-macos-intel-v0.0.1.tar.gz",
          "opengoal-windows-v0.0.1.zip",
          "opengoal-linux-v0.0.1.tar.gz",
        ]),
      ]),
    );
    const releases = await listOfficialReleases();
    expect(releases.length).toBe(1);
    expect(
      releases[0].downloadUrl?.endsWith("opengoal-windows-v0.0.1.zip"),
    ).toBeTruthy();
  });

  it("should retrieve linux releases properly", async () => {
    vi.mocked(platform).mockImplementation(() => "linux");
    vi.mocked(arch).mockImplementation(() => "x86_64");
    (fetch as Mock).mockResolvedValue(
      createFetchResponse([
        createFakeGithubRelease([
          "opengoal-macos-intel-v0.0.1.tar.gz",
          "opengoal-windows-v0.0.1.zip",
          "opengoal-linux-v0.0.1.tar.gz",
        ]),
      ]),
    );
    const releases = await listOfficialReleases();
    expect(releases.length).toBe(1);
    expect(
      releases[0].downloadUrl?.endsWith("opengoal-linux-v0.0.1.tar.gz"),
    ).toBeTruthy();
  });
});
