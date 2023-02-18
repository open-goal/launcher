export interface OfficialRelease {
  version: string;
  date: string | undefined;
  githubLink: string | undefined;
  downloadUrl: string | undefined; // TODO - windows/mac/linux
  isDownloaded: boolean;
}

export async function listOfficialReleases(): Promise<OfficialRelease[]> {
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
      version: release.tag_name,
      date: release.published_at,
      githubLink: release.html_url,
      // TODO A HACK
      downloadUrl:
        "https://github.com/open-goal/jak-project/releases/download/v0.1.32/opengoal-windows-v0.1.32.zip",
      isDownloaded: false,
    });
  }

  return releases.sort((a, b) => b.date.localeCompare(a.date));
}

export async function getLatestOfficialRelease(): Promise<OfficialRelease> {
  // TODO - handle rate limiting
  // TODO - even longer term - extract this out into an API we control (avoid github rate limiting) -- will be needed for unofficial releases as well anyway
  const resp = await fetch(
    "https://api.github.com/repos/open-goal/jak-project/releases/latest"
  );
  // TODO - handle error
  const githubRelease = await resp.json();
  return {
    version: githubRelease.tag_name,
    date: githubRelease.published_at,
    githubLink: githubRelease.html_url,
    // TODO - HACK
    downloadUrl: undefined,
    isDownloaded: false,
  };
}
