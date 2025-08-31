import { getModSources, getModSourcesData } from "$lib/rpc/features";
import type { PageLoad } from "./$types";

export const load = (async () => {
  const sources = await getModSources();
  return { sources };
}) satisfies PageLoad;
