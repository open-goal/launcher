import type { PageLoad } from "./$types";

export const load = (async ({ parent }) => {
  const { config } = await parent();
  const shouldAutoUpdate = config.autoUpdateGames;
  return {};
}) satisfies PageLoad;
