import { arch, platform } from "@tauri-apps/api/os";

export interface ReleaseInfo {
  releaseType: "official" | "unofficial" | "devel";
  version: string;
  date: string | undefined;
  githubLink: string | undefined;
  downloadUrl: string | undefined;
  isDownloaded: boolean;
  pendingAction: boolean;
}

async function getDownloadLinkForCurrentPlatform(
  release
): Promise<string | undefined> {
  const platformName = await platform();
  const archName = await arch();
  for (const asset of release.assets) {
    if (
      platformName === "darwin" &&
      archName === "x86_64" &&
      asset.name.startsWith("opengoal-macos-intel-v")
      // macOS doesn't have the old naming scheme
    ) {
      return asset.browser_download_url;
    } else if (
      platformName === "win32" &&
      (asset.name.startsWith("opengoal-windows-v") ||
        (asset.name.startsWith("opengoal-v") && asset.name.includes("windows")))
    ) {
      return asset.browser_download_url;
    } else if (
      platformName === "linux" &&
      (asset.name.startsWith("opengoal-linux-v") ||
        (asset.name.startsWith("opengoal-v") && asset.name.includes("linux")))
    ) {
      return asset.browser_download_url;
    }
  }
  return undefined;
}

export async function listOfficialReleases(): Promise<ReleaseInfo[]> {
  let releases = [];
  // TODO - handle rate limiting
  // TODO - long term - handle pagination (more than 100 releases)
  // TODO - even longer term - extract this out into an API we control (avoid github rate limiting) -- will be needed for unofficial releases as well anyway
  const resp = await fetch(
    "https://api.github.com/repos/open-goal/jak-project/releases?per_page=100"
  );
  // TODO - handle error
  const githubReleases = await resp.json();

  for (const release of githubReleases) {
    releases.push({
      releaseType: "official",
      version: release.tag_name,
      date: release.published_at,
      githubLink: release.html_url,
      downloadUrl: await getDownloadLinkForCurrentPlatform(release),
      isDownloaded: false,
      pendingAction: false,
    });
  }

  return releases.sort((a, b) => b.date.localeCompare(a.date));
}

export async function getLatestOfficialRelease(): Promise<ReleaseInfo> {
  // TODO - handle rate limiting
  // TODO - even longer term - extract this out into an API we control (avoid github rate limiting) -- will be needed for unofficial releases as well anyway
  const resp = await fetch(
    "https://api.github.com/repos/open-goal/jak-project/releases/latest"
  );
  // TODO - handle error
  const githubRelease = await resp.json();
  return {
    releaseType: "official",
    version: githubRelease.tag_name,
    date: githubRelease.published_at,
    githubLink: githubRelease.html_url,
    downloadUrl: await getDownloadLinkForCurrentPlatform(githubRelease),
    isDownloaded: false,
    pendingAction: false,
  };
}
