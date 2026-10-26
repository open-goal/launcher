import { toastStore } from "$lib/stores/ToastStore";
import { arch, platform } from "@tauri-apps/plugin-os";
import { unwrapFunctionStore, format } from "svelte-i18n";
import type { RestEndpointMethodTypes } from "@octokit/rest";

type GitHubRelease =
  RestEndpointMethodTypes["repos"]["getRelease"]["response"]["data"];

const $format = unwrapFunctionStore(format);

export interface ReleaseInfo {
  version: string;
  date: string | undefined;
  githubLink: string | undefined;
  downloadUrl: string | undefined;
  isDownloaded: boolean;
  pendingAction: boolean;
  invalid: boolean;
  invalidationReasons: string[];
}

function getDownloadLinkForCurrentPlatform(githubRelease: GitHubRelease) {
  let plat = platform();
  let matchingAsset;
  if (plat == "macos") {
    const userArch = arch() === "aarch64" ? "arm" : "intel";
    matchingAsset = githubRelease.assets.find(
      (asset) =>
        asset.name.toLowerCase().includes(plat) &&
        !asset.name.toLowerCase().includes(".bin") &&
        !asset.name.toLowerCase().includes("lsp") &&
        asset.name.toLowerCase().includes(userArch),
    );
  } else {
    matchingAsset = githubRelease.assets.find(
      (asset) =>
        asset.name.toLowerCase().includes(plat) &&
        !asset.name.toLowerCase().includes(".bin") &&
        !asset.name.toLowerCase().includes("lsp"),
    );
  }
  if (matchingAsset) {
    return matchingAsset.browser_download_url;
  }
  return undefined;
}

async function parseGithubRelease(
  githubRelease: GitHubRelease,
): Promise<ReleaseInfo> {
  const releaseInfo: ReleaseInfo = {
    version: githubRelease.tag_name,
    date:
      githubRelease.published_at === null
        ? undefined
        : githubRelease.published_at,
    githubLink: githubRelease.html_url,
    downloadUrl: getDownloadLinkForCurrentPlatform(githubRelease),
    isDownloaded: false,
    pendingAction: false,
    invalid: false,
    invalidationReasons: [],
  };
  if (githubRelease.body && githubRelease.body.includes("<!-- invalid:")) {
    releaseInfo.invalid = true;
    // Get the line it's on
    try {
      const line = githubRelease.body
        .split("<!-- invalid:")[1]
        .split("-->")[0]
        .trim();
      releaseInfo.invalidationReasons = line.split("|");
    } catch (err) {
      // do nothing, bad formatting
      releaseInfo.invalidationReasons = ["Release invalid for unknown reasons"];
    }
  }

  return releaseInfo;
}

// Fetches every release for a repo (ie. "open-goal/jak-project"), following pagination
async function fetchGithubReleases(repo: string): Promise<GitHubRelease[]> {
  const nextUrlPattern = /<([\S]+)>; rel="Next"/i;
  let releases: GitHubRelease[] = [];
  let urlToHit: string | undefined =
    `https://api.github.com/repos/${repo}/releases?per_page=100`;

  while (urlToHit !== undefined) {
    const resp: Response = await fetch(urlToHit);
    if (resp.status === 429) {
      toastStore.makeToast($format("toasts_githubRateLimit"), "error");
      return [];
    } else if (!resp.ok) {
      toastStore.makeToast(resp.statusText, "error");
      return [];
    }

    const githubReleases: GitHubRelease[] = await resp.json();
    releases.push(...githubReleases);

    const linkHeader = resp.headers.get("link");
    if (linkHeader && linkHeader.includes(`rel=\"next\"`)) {
      const match = linkHeader.match(nextUrlPattern);
      if (match && match.length >= 2) {
        // we must paginate!
        urlToHit = match[1];
      } else {
        urlToHit = undefined;
      }
    } else {
      urlToHit = undefined;
    }
  }
  return releases;
}

function sortByDateDescending<T extends { date: string | undefined }>(
  releases: T[],
): T[] {
  return releases.sort((a, b) => {
    const aDate = a.date;
    const bDate = b.date;
    if (aDate && bDate) {
      return bDate.localeCompare(aDate);
    }
    return 0;
  });
}

export async function listOfficialReleases(): Promise<ReleaseInfo[]> {
  let releases = [];
  for (const release of await fetchGithubReleases("open-goal/jak-project")) {
    releases.push(await parseGithubRelease(release));
  }
  return sortByDateDescending(releases);
}

export interface ChangelogEntry {
  contributor: string;
  description: string;
  pullRequestUrl: string;
}

export interface LauncherRelease {
  version: string;
  date: string | undefined;
  githubLink: string;
  changes: ChangelogEntry[];
}

export function isDependencyChange(entry: ChangelogEntry): boolean {
  return entry.contributor.includes("dependabot");
}

// Parses the auto-generated "What's Changed" lines out of a release's markdown body, ie.
// * jak1/speedruns: Some final touches by @open-goal in https://github.com/open-goal/jak-project/pull/1830
// Mirrors `changesFromBody` in scripts/update-release-meta.mjs
export function changesFromReleaseBody(releaseBody: string): ChangelogEntry[] {
  let changes: ChangelogEntry[] = [];
  for (const line of releaseBody.split("\n")) {
    // check if the string contains a pull request link
    if (!line.toLowerCase().match(/.*github.com\/[^/]*\/[^/]*\/pull.*/)) {
      continue;
    }
    let description = [];
    let contributor = "";
    let pullRequestUrl = "";
    const words = line
      .trim()
      .replace(/^(\* )/, "")
      .split(" ");
    let i = 0;
    while (i < words.length) {
      if (words[i] == "by" && i + 1 < words.length) {
        contributor = words[i + 1].replace(/^(@)/, "");
        i += 2;
      } else if (words[i] == "in" && i + 1 < words.length) {
        pullRequestUrl = words[i + 1];
        i += 2;
      } else {
        description.push(words[i]);
        i++;
      }
    }
    if (!changes.some((entry) => entry.pullRequestUrl === pullRequestUrl)) {
      changes.push({
        contributor,
        description: description.join(" "),
        pullRequestUrl,
      });
    }
  }
  return changes;
}

export async function listLauncherReleases(): Promise<LauncherRelease[]> {
  const releases = (await fetchGithubReleases("open-goal/launcher")).map(
    (release) => ({
      version: release.tag_name,
      date: release.published_at ?? undefined,
      githubLink: release.html_url,
      changes: changesFromReleaseBody(release.body ?? ""),
    }),
  );
  return sortByDateDescending(releases);
}

export async function getLatestOfficialRelease(): Promise<
  ReleaseInfo | undefined
> {
  const resp = await fetch(
    "https://api.github.com/repos/open-goal/jak-project/releases/latest",
  );
  if (resp.status === 429) {
    toastStore.makeToast($format("toasts_githubRateLimit"), "error");
    return undefined;
  } else if (!resp.ok) {
    toastStore.makeToast(resp.statusText, "error");
    return undefined;
  }
  const githubRelease = await resp.json();
  return await parseGithubRelease(githubRelease);
}

// Releases newer than `version` (ie. "2.11.0" or "v2.11.0"), newest first
// Empty if `version` isn't a published release (ie. a dev build) or GitHub couldn't be reached
export async function listLauncherReleasesSince(
  version: string,
): Promise<LauncherRelease[]> {
  const releases = await listLauncherReleases();
  const currentIndex = releases.findIndex(
    (release) =>
      release.version.replace(/^v/, "") === version.replace(/^v/, ""),
  );
  return currentIndex === -1 ? [] : releases.slice(0, currentIndex);
}
