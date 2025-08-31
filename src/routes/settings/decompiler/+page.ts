import { doesActiveToolingVersionMeetMinimum } from "$lib/rpc/config";
import type { PageLoad } from "./$types";

export const load = (async () => {
  const allowed = await doesActiveToolingVersionMeetMinimum(0, 2, 14);
  return { allowed };
}) satisfies PageLoad;
