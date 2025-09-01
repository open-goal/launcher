import type { LayoutLoad } from "./$types";

export const load = (async ({ params }) => {
  const game = params.game;

  return { game };
}) satisfies LayoutLoad;
