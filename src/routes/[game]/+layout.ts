import type { LayoutLoad } from "./$types";
import { saveActiveVersionChange } from "$lib/rpc/config";
import { downloadLatestOfficialVersion } from "$lib/rpc/versions";
import { getLatestOfficialRelease } from "$lib/utils/github";
import { invalidateAll } from "$app/navigation";

export const load = (async ({ params, parent }) => {
  const game = params.game;
  const { config } = await parent();

  // INSTALL LATEST TOOLING VERSION IF NONE SET
  // needs some work to be fully functional, need a global store for if were already downloading
  // if (!config.activeVersion) {
  //   console.log("downloading the latest release");
  //   const release = await getLatestOfficialRelease();
  //   await downloadLatestOfficialVersion();
  //   await saveActiveVersionChange(release);
  //   await invalidateAll();
  // }

  return { game };
}) satisfies LayoutLoad;
