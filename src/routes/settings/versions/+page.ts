import { listDownloadedVersions } from "$lib/rpc/versions";
import { listOfficialReleases, type ReleaseInfo } from "$lib/utils/github";
import type { PageLoad } from "./$types";

function hasDownloadUrl(
  r: ReleaseInfo,
): r is ReleaseInfo & { downloadUrl: string } {
  return typeof r.downloadUrl === "string" && r.downloadUrl.length > 0;
}

function isValid(r: ReleaseInfo) {
  return !r.invalid;
}

export const load = (async () => {
  const [officialRaw, installed] = await Promise.all([
    listOfficialReleases(),
    listDownloadedVersions(),
  ]);
  const official = officialRaw.filter(hasDownloadUrl).filter(isValid);

  return { official, installed };
}) satisfies PageLoad;
