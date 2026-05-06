import { locale as svelteLocale } from "svelte-i18n";
import { invoke_rpc, invoke_rpc2 } from "./rpc";
import { AVAILABLE_LOCALES, type Locale } from "$lib/i18n/i18n";
import { exists } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { SupportedGame } from "./bindings/SupportedGame";
import type { LauncherConfig } from "./bindings/LauncherConfig";

export async function resetLauncherSettings(): Promise<string | null> {
  return await invoke_rpc("reset_to_defaults", {});
}

export async function getLauncherConfig(): Promise<LauncherConfig | null> {
  return await invoke_rpc("get_launcher_config", {});
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
  return await invoke_rpc("set_game_installed", {
    gameName,
    installed,
  });
}

export async function setActiveVersion(version: String): Promise<boolean> {
  return invoke_rpc("set_active_version", { version: version }, () => true);
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
    "set_locale",
    { locale: localeId },
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

export async function setAutoUninstallOldVersions(
  value: boolean,
): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "delete_previous_versions",
    val: value,
  });
}

export async function setBypassRequirements(bypass: boolean): Promise<void> {
  return await invoke_rpc("set_bypass_requirements", {
    bypass: bypass,
  });
}

export async function setCheckForLatestModVersion(
  check_for_latest_mod_version: boolean,
): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "check_for_latest_mod_version",
    val: check_for_latest_mod_version,
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
  return await invoke_rpc("update_setting_value", {
    key: "rip_levels",
    val: enabled,
  });
}

export async function setRipCollisionEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "rip_collision",
    val: enabled,
  });
}
export async function setRipTexturesEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc("update_setting_value", {
    key: "rip_textures",
    val: enabled,
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
