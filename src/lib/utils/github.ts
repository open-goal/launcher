import { toastStore } from "$lib/stores/ToastStore";
import { arch, platform } from "@tauri-apps/api/os";
import { unwrapFunctionStore, format } from "svelte-i18n";

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

function isIntelMacOsRelease(
  platform: string,
  architecture: string,
  assetName: string,
): boolean {
  return (
    platform === "darwin" &&
    architecture === "x86_64" &&
    assetName.startsWith("opengoal-macos-intel-v")
  );
}

function isWindowsRelease(
  platform: string,
  architecture: string,
  assetName: string,
): boolean {
  return platform === "win32" && assetName.startsWith("opengoal-windows-v");
}

function isLinuxRelease(
  platform: string,
  architecture: string,
  assetName: string,
): boolean {
  return platform === "linux" && assetName.startsWith("opengoal-linux-v");
}

async function getDownloadLinkForCurrentPlatform(
  release: any,
): Promise<string | undefined> {
  const platformName = await platform();
  const archName = await arch();
  for (const asset of release.assets) {
    if (isIntelMacOsRelease(platformName, archName, asset.name)) {
      return asset.browser_download_url;
    } else if (isWindowsRelease(platformName, archName, asset.name)) {
      return asset.browser_download_url;
    } else if (isLinuxRelease(platformName, archName, asset.name)) {
      return asset.browser_download_url;
    }
  }
  return undefined;
}

async function parseGithubRelease(githubRelease: any): Promise<ReleaseInfo> {
  const releaseInfo: ReleaseInfo = {
    releaseType: "official",
    version: githubRelease.tag_name,
    date: githubRelease.published_at,
    githubLink: githubRelease.html_url,
    downloadUrl: await getDownloadLinkForCurrentPlatform(githubRelease),
    isDownloaded: false,
    pendingAction: false,
    invalid: false,
    invalidationReasons: [],
  };
  if (githubRelease.body.includes("<!-- invalid:")) {
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

export async function listOfficialReleases(): Promise<ReleaseInfo[]> {
  const nextUrlPattern = /<([\S]+)>; rel="Next"/i;
  let releases = [];
  let urlToHit =
    "https://api.github.com/repos/open-goal/jak-project/releases?per_page=100";

  while (urlToHit !== undefined) {
    const resp = await fetch(urlToHit);
    if (resp.status === 403 || resp.status === 429) {
      toastStore.makeToast($format("toasts_githubRateLimit"), "error");
      return [];
    } else if (!resp.ok) {
      toastStore.makeToast($format("toasts_githubUnexpectedError"), "error");
      return [];
    }

    const githubReleases = await resp.json();
    for (const release of githubReleases) {
      releases.push(await parseGithubRelease(release));
    }

    if (
      resp.headers.has("link") &&
      resp.headers.get("link").includes(`rel=\"next\"`)
    ) {
      // we must paginate!
      urlToHit = resp.headers.get("link").match(nextUrlPattern)[1];
    } else {
      urlToHit = undefined;
    }
  }

  return releases.sort((a, b) => b.date.localeCompare(a.date));
}

export async function getLatestOfficialRelease(): Promise<
  ReleaseInfo | undefined
> {
  const resp = await fetch(
    "https://api.github.com/repos/open-goal/jak-project/releases/latest",
  );
  if (resp.status === 403 || resp.status === 429) {
    toastStore.makeToast($format("toasts_githubRateLimit"), "error");
    return undefined;
  } else if (!resp.ok) {
    toastStore.makeToast($format("toasts_githubUnexpectedError"), "error");
    return undefined;
  }
  const githubRelease = await resp.json();
  return await parseGithubRelease(githubRelease);
}
