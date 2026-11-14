import {
  locale as svelteLocale,
  unwrapFunctionStore,
  format,
} from "svelte-i18n";
import { invoke_rpc, invoke_rpc2 } from "./rpc";
import {
  AVAILABLE_LOCALES,
  applyLocaleFont,
  type Locale,
} from "$lib/i18n/i18n";
import { exists } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import type { LauncherConfig } from "./bindings/LauncherConfig";
import { toastStore } from "$lib/stores/ToastStore";

const $format = unwrapFunctionStore(format);

export async function resetLauncherSettings(): Promise<void> {
  return await invoke_rpc("reset_to_defaults");
}

export async function getLauncherConfig(): Promise<LauncherConfig | null> {
  return await invoke_rpc("get_launcher_config");
}

export async function setInstallationDirectory(newDir: string): Promise<void> {
  return await invoke_rpc("set_install_directory", { newDir: newDir });
}

export async function isAVXRequirementMet(): Promise<boolean | undefined> {
  return await invoke_rpc("is_avx_requirement_met");
}

export async function isOpenGLRequirementMet(
  force: boolean,
): Promise<boolean | undefined> {
  return await invoke_rpc("is_opengl_requirement_met", { force });
}

export async function isDiskSpaceRequirementMet(
  gameName: string,
): Promise<boolean | undefined> {
  return await invoke_rpc("is_diskspace_requirement_met", { gameName });
}

export async function isMinimumVCCRuntimeInstalled(): Promise<boolean> {
  return await invoke_rpc("is_minimum_vcc_runtime_installed");
}

export async function finalizeInstallation(
  gameName: string,
  installed = true,
): Promise<void> {
  return await invoke_rpc("set_game_installed", {
    gameName,
    installed,
  });
}

export async function setActiveVersion(version: string): Promise<void> {
  await invoke_rpc("set_active_version", { version: version });
  toastStore.makeToast($format("toasts_savedToolingVersion"), "info");
}

export async function localeFontDownloadAvailable(
  localeId: string,
): Promise<Locale | undefined> {
  const locale = AVAILABLE_LOCALES.find(({ id }) => id === localeId);
  if (!locale?.fontFileName || !locale.fontDownloadUrl) return;

  const fontPath = await join(await appDataDir(), "fonts", locale.fontFileName);

  return (await exists(fontPath)) ? undefined : locale;
}

export async function setHideBetaAlerts(hide: boolean): Promise<void> {
  return await invoke_rpc("set_hide_beta_alerts", {
    hide: hide,
  });
}

export async function setHideGamename(hide: boolean): Promise<void> {
  return await invoke_rpc("set_hide_gamename", {
    hide: hide,
  });
}

export async function setHidePlaytime(hide: boolean): Promise<void> {
  return await invoke_rpc("set_hide_playtime", {
    hide: hide,
  });
}

export async function setLocale(localeId: string): Promise<void> {
  await invoke_rpc("set_locale", { locale: localeId });
  svelteLocale.set(localeId);
  await applyLocaleFont(localeId);
}

export async function setAutoUpdateGames(value: boolean): Promise<void> {
  return await invoke_rpc("set_auto_update_games", {
    autoUpdate: value,
  });
}

export async function setAutoUninstallOldVersions(
  value: boolean,
): Promise<void> {
  return await invoke_rpc("set_delete_previous_versions", {
    delete: value,
  });
}

export async function setBypassRequirements(bypass: boolean): Promise<void> {
  return await invoke_rpc("set_bypass_requirements", {
    bypass: bypass,
  });
}

export async function setCheckForLatestModVersion(
  check: boolean,
): Promise<void> {
  return await invoke_rpc("set_check_for_latest_mod_version", {
    check: check,
  });
}

export async function cleanupEnabledTexturePacks(
  gameName: string,
  cleanupList: string[],
): Promise<void> {
  return await invoke_rpc("cleanup_enabled_texture_packs", {
    gameName: gameName,
    cleanupList: cleanupList,
  });
}

export async function setEnabledTexturePacks(
  gameName: string,
  packs: string[],
): Promise<string | null> {
  return await invoke_rpc2("set_texture_packs", {
    args: {
      gameName: gameName,
      texturePacks: packs,
    },
  });
}

export async function doesActiveToolingVersionSupportGame(
  gameName: string,
): Promise<boolean> {
  return await invoke_rpc("does_active_tooling_version_support_game", {
    gameName: gameName,
  });
}

export async function doesActiveToolingVersionMeetMinimum(
  minimumMajor: number,
  minimumMinor: number,
  minimumPatch: number,
): Promise<boolean> {
  return await invoke_rpc("does_active_tooling_version_meet_minimum", {
    minimumPatch: minimumPatch,
    minimumMinor: minimumMinor,
    minimumMajor: minimumMajor,
  });
}

export async function setRipLevelsEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("set_rip_levels", {
    enabled: enabled,
  });
}

export async function setRipCollisionEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("set_rip_collision", {
    enabled: enabled,
  });
}
export async function setRipTexturesEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("set_rip_textures", {
    enabled: enabled,
  });
}

export async function setRipStreamedAudioEnabled(
  enabled: boolean,
): Promise<void> {
  return await invoke_rpc("set_rip_streamed_audio", {
    enabled: enabled,
  });
}
export async function setRipMusicEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("set_rip_music", {
    enabled: enabled,
  });
}
