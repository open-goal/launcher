import { locale as svelteLocale } from "svelte-i18n";
import { invoke_rpc, invoke_rpc2 } from "./rpc";
import { AVAILABLE_LOCALES, type Locale } from "$lib/i18n/i18n";
import { exists } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { SupportedGame } from "./bindings/SupportedGame";

export async function resetLauncherSettingsToDefaults(): Promise<boolean> {
  return await invoke_rpc("reset_to_defaults", {});
}

export async function getInstallationDirectory(): Promise<string | null> {
  return await invoke_rpc("get_setting_value", { key: "install_directory" });
}

export async function setInstallationDirectory(
  newDir: string,
): Promise<string | null> {
  return await invoke_rpc("set_install_directory", { newDir: newDir });
}

export async function isAVXRequirementMet(): Promise<boolean | undefined> {
  return await invoke_rpc("is_avx_requirement_met", {});
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
  return await invoke_rpc("is_minimum_vcc_runtime_installed", {});
}

export async function finalizeInstallation(
  gameName: string,
  installed = true,
): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "installed",
    val: installed,
    gameName,
  });
}

export async function isGameInstalled(
  gameName: SupportedGame | string,
): Promise<boolean> {
  return await invoke_rpc("get_setting_value", { key: "installed", gameName });
}

export async function getInstalledVersion(
  gameName: string,
): Promise<String | undefined> {
  return invoke_rpc("get_setting_value", {
    key: "installed_version",
    gameName: gameName,
  });
}

export async function saveActiveVersionChange(
  newActiveVersion: String,
): Promise<boolean> {
  return invoke_rpc(
    "update_setting_value",
    { key: "active_version", val: newActiveVersion },
    () => true,
  );
}

export async function getLocale(): Promise<string | null> {
  return await invoke_rpc("get_setting_value", { key: "locale" });
}

export async function localeSpecificFontAvailableForDownload(
  localeId: string,
): Promise<Locale | undefined> {
  let localeInfo = AVAILABLE_LOCALES.find((locale) => locale.id === localeId);
  if (
    localeInfo !== undefined &&
    localeInfo.fontFileName !== undefined &&
    localeInfo.fontDownloadUrl !== undefined
  ) {
    const fontPath = await join(
      await appDataDir(),
      "fonts",
      localeInfo.fontFileName,
    );
    const fontAlreadyDownloaded = await exists(fontPath);
    if (fontAlreadyDownloaded) {
      return undefined;
    }
    return localeInfo;
  }
  return undefined;
}

export async function setLocale(localeId: string): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "locale", val: localeId },
    undefined,
    async () => {
      svelteLocale.set(localeId);
      // Update CSS variable if needed
      let localeInfo = AVAILABLE_LOCALES.find(
        (locale) => locale.id === localeId,
      );
      if (
        localeInfo !== undefined &&
        localeInfo.fontFamily !== undefined &&
        localeInfo.fontFileName !== undefined
      ) {
        // Dynamically get the font
        const fontPath = await join(
          await appDataDir(),
          "fonts",
          localeInfo.fontFileName,
        );
        const fontExists = await exists(fontPath);
        if (fontExists) {
          const assetUrl = convertFileSrc(fontPath);
          var newFontStyle = document.createElement("style");
          newFontStyle.appendChild(
            document.createTextNode(
              `@font-face {\nfont-family: "${localeInfo.fontFamily}";\nsrc: url('${assetUrl}');\n}\n`,
            ),
          );
          document.head.appendChild(newFontStyle);
          document.documentElement.style.setProperty(
            "--launcher-font-family",
            localeInfo.fontFamily,
          );
        } else {
          document.documentElement.style.setProperty(
            "--launcher-font-family",
            "Noto Sans",
          );
        }
      } else {
        document.documentElement.style.setProperty(
          "--launcher-font-family",
          "Noto Sans",
        );
      }
    },
  );
}

export async function setAutoUpdateGames(value: boolean): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "auto_update_games",
    val: value,
  });
}

export async function getAutoUpdateGames(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", { key: "auto_update_games" });
}

export async function setAutoUninstallOldVersions(
  value: boolean,
): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "delete_previous_versions",
    val: value,
  });
}

export async function getAutoUninstallOldVersions(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", {
    key: "delete_previous_versions",
  });
}

export async function setBypassRequirements(bypass: boolean): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "bypass_requirements",
    val: bypass,
  });
}

export async function getBypassRequirements(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", { key: "bypass_requirements" });
}

export async function setCheckForLatestModVersion(
  check_for_latest_mod_version: boolean,
): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "check_for_latest_mod_version",
    val: check_for_latest_mod_version,
  });
}

export async function getCheckForLatestModVersion(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", {
    key: "check_for_latest_mod_version",
  });
}

export async function getEnabledTexturePacks(
  gameName: string,
): Promise<string[]> {
  return await invoke_rpc("get_setting_value", {
    key: "active_texture_packs",
    gameName: gameName,
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
  return await invoke_rpc2("update_mods_setting_value", {
    args: {
      key: "add_texture_packs",
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

export async function getPlaytime(gameName: string): Promise<number> {
  return await invoke_rpc("get_setting_value", {
    key: "seconds_played",
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

export async function isRipLevelsEnabled(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", { key: "rip_levels" });
}

export async function setRipLevelsEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "rip_levels",
    val: enabled,
  });
}

export async function isRipCollisionEnabled(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", { key: "rip_collision" });
}

export async function setRipCollisionEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "rip_collision",
    val: enabled,
  });
}

export async function isRipTexturesEnabled(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", { key: "rip_textures" });
}

export async function setRipTexturesEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "rip_textures",
    val: enabled,
  });
}

export async function isRipStreamedAudioEnabled(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", { key: "rip_streamed_audio" });
}

export async function getProceedAfterSuccessfulOperation(): Promise<boolean> {
  return await invoke_rpc("get_setting_value", {
    key: "proceed_after_successful_operation",
  });
}

export async function setRipStreamedAudioEnabled(
  enabled: boolean,
): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "rip_streamed_audio",
    val: enabled,
  });
}
