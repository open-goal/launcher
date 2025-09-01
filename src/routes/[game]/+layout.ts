import {
  isAVXRequirementMet,
  isDiskSpaceRequirementMet,
  isMinimumVCCRuntimeInstalled,
  isOpenGLRequirementMet,
} from "$lib/rpc/config";
import { type } from "@tauri-apps/plugin-os";
import type { LayoutLoad } from "./$types";
import { redirect } from "@sveltejs/kit";

export const load = (async ({ params }) => {
  const game = params.game;
  const osRelevant = type() !== "macos";
  const isAVXMet = osRelevant ? await isAVXRequirementMet() : true;
  const isOpenGLMet = await isOpenGLRequirementMet(false);
  const isDiskSpaceMet = await isDiskSpaceRequirementMet(game);
  const vcc = osRelevant ? await isMinimumVCCRuntimeInstalled() : true;
  const requirementsMet = isAVXMet && isOpenGLMet && isDiskSpaceMet && vcc;

  if (!requirementsMet) {
    const q = new URLSearchParams({
      game,
      avx: isAVXMet,
      openGL: isOpenGLMet,
      disk: isDiskSpaceMet,
      vcc: vcc,
    });
    throw redirect(308, `/requirements?${q}`);
  }

  return { game };
}) satisfies LayoutLoad;
