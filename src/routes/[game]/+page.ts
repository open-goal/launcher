import { appDataDir, configDir, join } from "@tauri-apps/api/path";
import type { PageLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import {
  doesActiveToolingVersionMeetMinimum,
  doesActiveToolingVersionSupportGame,
  isAVXRequirementMet,
  isDiskSpaceRequirementMet,
  isMinimumVCCRuntimeInstalled,
  isOpenGLRequirementMet,
} from "$lib/rpc/config";
import { type } from "@tauri-apps/plugin-os";
import { ensureActiveVersionStillExists } from "$lib/rpc/versions";
import { exists } from "@tauri-apps/plugin-fs";
import { convertFileSrc } from "@tauri-apps/api/core";

export const load = (async ({ parent, params }) => {
  const game = params.game;
  const { config } = await parent();
  const gameConfig = config.games[game];

  const supported = await doesActiveToolingVersionSupportGame(game);
  if (!supported) {
    throw redirect(308, `/${game}/unsupported`);
  }

  const installed = gameConfig.isInstalled;
  if (!installed) {
    throw redirect(308, `/${game}/install`);
  }

  const versionExists = await ensureActiveVersionStillExists();
  if (!versionExists) {
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

  const texturesSupported = await doesActiveToolingVersionMeetMinimum(0, 2, 13);

  let bgVideo = null;
  const appDataDirPath = await appDataDir();
  const bgVideoPath = await join(appDataDirPath, "backgrounds", `${game}.mp4`);
  if (await exists(bgVideoPath)) {
    bgVideo = convertFileSrc(bgVideoPath);
  }

  return {
    gameDataDir,
    extractedAssetsDir,
    savesDir,
    settingsDir,
    texturesSupported,
    bgVideo,
  };
}) satisfies PageLoad;
