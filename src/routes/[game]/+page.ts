import { configDir, join } from "@tauri-apps/api/path";
import type { PageLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import {
  isAVXRequirementMet,
  isDiskSpaceRequirementMet,
  isMinimumVCCRuntimeInstalled,
  isOpenGLRequirementMet,
} from "$lib/rpc/config";
import { type } from "@tauri-apps/plugin-os";

export const load = (async ({ parent, params }) => {
  const game = params.game;
  const { config } = await parent();
  const gameConfig = config.games[game];

  const installed = gameConfig.isInstalled;
  if (!installed) {
    throw redirect(308, `/${game}/install`);
  }

  const activeVersion = config.activeVersion;
  const installedVersion = gameConfig.version;
  const outdated = activeVersion !== installedVersion;
  if (outdated) {
    throw redirect(308, `/${game}/outdated`);
  }

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

  const gameDataDir = await join(
    config.installationDir,
    "active",
    game,
    "data",
  );
  const savesDir = await join(await configDir(), "OpenGOAL", game, "saves");
  const extractedAssetsDir = await join(gameDataDir, "decompiler_out", game);
  const settingsDir = await join(
    await configDir(),
    "OpenGOAL",
    game,
    "settings",
  );

  return { gameDataDir, extractedAssetsDir, savesDir, settingsDir };
}) satisfies PageLoad;
