import { toastStore } from "$lib/stores/ToastStore";
import { locale as svelteLocale } from "svelte-i18n";
import { errorLog } from "./logging";
import { invoke_rpc } from "./rpc";
import { AVAILABLE_LOCALES, type Locale } from "$lib/i18n/i18n";
import { exists } from "@tauri-apps/api/fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";

export async function oldDataDirectoryExists(): Promise<boolean> {
  return await invoke_rpc("has_old_data_directory", {}, () => false);
}

export async function deleteOldDataDirectory(): Promise<void> {
  return await invoke_rpc("delete_old_data_directory", {}, () => {});
}

export async function resetLauncherSettingsToDefaults(): Promise<boolean> {
  const success = await invoke_rpc(
    "reset_to_defaults",
    {},
    () => false,
    "Unable to reset settings",
  );
  return success != false;
}

export async function getInstallationDirectory(): Promise<string | null> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "install_directory" },
    () => null,
  );
}

export async function setInstallationDirectory(
  newDir: string,
): Promise<string | null> {
  // TODO - not insanely crazy about this pattern (message in the response instead of the error)
  // consider changing it
  const errMsg = await invoke_rpc(
    "set_install_directory",
    { newDir: newDir },
    () => "Unexpected error occurred",
    "Invalid installation directory",
  );

  if (errMsg !== null) {
    // for RPC errors the log and toast are done by invoke_rpc
    // but this is a successful RPC, so we need to do it here
    errorLog("Unable to set install directory");
    toastStore.makeToast(errMsg, "error");
  }

  return errMsg;
}

export async function isAVXRequirementMet(): Promise<boolean | undefined> {
  return await invoke_rpc("is_avx_requirement_met", {}, () => undefined);
}

export async function isOpenGLRequirementMet(
  force: boolean,
): Promise<boolean | undefined> {
  const result = await invoke_rpc(
    "is_opengl_requirement_met",
    { force },
    () => undefined,
  );

  if (typeof result !== "boolean") {
    return undefined;
  }

  return result;
}

export async function isDiskSpaceRequirementMet(
  gameName: string,
): Promise<boolean | undefined> {
  return await invoke_rpc(
    "is_diskspace_requirement_met",
    { gameName },
    () => undefined,
  );
}

export async function isMinimumVCCRuntimeInstalled(): Promise<
  boolean | undefined
> {
  return await invoke_rpc(
    "is_minimum_vcc_runtime_installed",
    {},
    () => undefined,
  );
}

export async function finalizeInstallation(
  gameName: string,
  installed = true,
): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "installed", val: installed, gameName },
    () => {},
  );
}

export async function isGameInstalled(gameName: string): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "installed", gameName },
    () => false,
  );
}

export async function getInstalledVersion(gameName: string): Promise<String> {
  return invoke_rpc(
    "get_setting_value",
    { key: "installed_version", gameName: gameName },
    () => null,
  );
}

export async function saveActiveVersionChange(
  newActiveVersion: String,
): Promise<boolean> {
  return invoke_rpc(
    "update_setting_value",
    { key: "active_version", val: newActiveVersion },
    () => false,
    "Couldn't save active version change",
    () => true,
  );
}

export async function getLocale(): Promise<string | null> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "locale" },
    () => "en-US",
  );
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
    () => {},
    undefined, // no toast
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
  return await invoke_rpc(
    "update_setting_value",
    { key: "auto_update_games", val: value },
    () => {},
  );
}

export async function getAutoUpdateGames(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "auto_update_games" },
    () => false,
  );
}

export async function setAutoUninstallOldVersions(
  value: boolean,
): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "delete_previous_versions", val: value },
    () => {},
  );
}

export async function getAutoUninstallOldVersions(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "delete_previous_versions" },
    () => false,
  );
}

export async function setBypassRequirements(bypass: boolean): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "bypass_requirements", val: bypass },
    () => {},
  );
}

export async function getBypassRequirements(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "bypass_requirements" },
    () => false,
  );
}

export async function setCheckForLatestModVersion(
  check_for_latest_mod_version: boolean,
): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "check_for_latest_mod_version", val: check_for_latest_mod_version },
    () => {},
  );
}

export async function getCheckForLatestModVersion(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "check_for_latest_mod_version" },
    () => false,
  );
}

export async function getEnabledTexturePacks(
  gameName: string,
): Promise<string[]> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "active_texture_packs", gameName: gameName },
    () => [],
  );
}

export async function cleanupEnabledTexturePacks(
  gameName: string,
  cleanupList: string[],
): Promise<void> {
  return await invoke_rpc(
    "cleanup_enabled_texture_packs",
    {
      gameName: gameName,
      cleanupList: cleanupList,
    },
    () => {},
  );
}

// TODO - just make this a generic interface for both binaries/feature jobs
interface FeatureJobOutput {
  msg: string | null;
  success: boolean;
}

function failed(msg: string): FeatureJobOutput {
  return { success: false, msg };
}

export async function setEnabledTexturePacks(
  gameName: string,
  packs: string[],
): Promise<FeatureJobOutput> {
  return await invoke_rpc(
    "update_mods_setting_value",
    {
      key: "add_texture_packs",
      gameName: gameName,
      texturePacks: packs,
    },
    () => failed("Failed to update texture pack list"),
    undefined,
    () => {
      return { success: true, msg: null };
    },
  );
}

export async function doesActiveToolingVersionSupportGame(
  gameName: string,
): Promise<boolean> {
  return await invoke_rpc(
    "does_active_tooling_version_support_game",
    {
      gameName: gameName,
    },
    () => false,
  );
}

export async function getPlaytime(gameName: string): Promise<number> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "seconds_played", gameName: gameName },
    () => 0,
  );
}

export async function doesActiveToolingVersionMeetMinimum(
  minimumMajor: number,
  minimumMinor: number,
  minimumPatch: number,
): Promise<boolean> {
  return await invoke_rpc(
    "does_active_tooling_version_meet_minimum",
    {
      minimumPatch: minimumPatch,
      minimumMinor: minimumMinor,
      minimumMajor: minimumMajor,
    },
    () => false,
  );
}

export async function isRipLevelsEnabled(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "rip_levels" },
    () => false,
  );
}

export async function setRipLevelsEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "rip_levels", val: enabled },
    () => {},
  );
}

export async function isRipCollisionEnabled(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "rip_collision" },
    () => false,
  );
}

export async function setRipCollisionEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "rip_collision", val: enabled },
    () => {},
  );
}

export async function isRipTexturesEnabled(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "rip_textures" },
    () => false,
  );
}

export async function setRipTexturesEnabled(enabled: boolean): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "rip_textures", val: enabled },
    () => {},
  );
}

export async function isRipStreamedAudioEnabled(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "rip_streamed_audio" },
    () => false,
  );
}

export async function getProceedAfterSuccessfulOperation(): Promise<boolean> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "proceed_after_successful_operation" },
    () => true,
  );
}

export async function setRipStreamedAudioEnabled(
  enabled: boolean,
): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "rip_streamed_audio", val: enabled },
    () => {},
  );
}
