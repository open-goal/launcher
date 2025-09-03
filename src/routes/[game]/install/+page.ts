import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { progressTracker } from "$lib/stores/ProgressStore";

export const load = (async ({ params, parent }) => {
  const { config, game } = await parent();
  const proceed = config.proceedAfterSuccessfulOperation;
  const installed = config.games[game].isInstalled;

  if (installed && proceed) {
    progressTracker.clear();
    throw redirect(308, `/${game}`);
  }

  return { proceed };
}) satisfies PageLoad;
