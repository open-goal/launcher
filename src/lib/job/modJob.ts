import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import {
  baseGameIsoExists,
  extractIsoForModInstall,
  decompileForModInstall,
  compileForModInstall,
  saveModInstallInfo,
  downloadAndExtractNewMod,
} from "$lib/rpc/features";
import { jobTracker } from "$lib/stores/JobStore";
import { isoPrompt } from "$lib/utils/file-dialogs";
import { unwrapFunctionStore, format } from "svelte-i18n";

const $format = unwrapFunctionStore(format);

export async function setupModInstallation(
  activeGame: SupportedGame,
  modName: string,
  modSourceName: string,
  modDownloadUrl: string | undefined,
  modVersion: string,
) {
  jobTracker.init([
    {
      status: "queued",
      label: $format("setup_extractAndVerify"),
      task: async () => {
        const isoAlreadyExtracted = await baseGameIsoExists(activeGame);
        if (!isoAlreadyExtracted) {
          let sourcePath = await isoPrompt(
            $format("setup_prompt_ISOFileLabel"),
            $format("setup_prompt_selectISO"),
          );
          if (sourcePath !== undefined) {
            let resp = await extractIsoForModInstall(
              activeGame,
              modName,
              modSourceName,
              sourcePath,
            );
            if (!resp.success) {
              jobTracker.updateFailureReason(resp.msg);
              return false;
            }
          } else {
            jobTracker.updateFailureReason($format("setup_error_noISO"));
            return false;
          }
        }
        if (modDownloadUrl) {
          // extract the file into install_dir/features/<game>/<sourceName>/<modName>
          let resp = await downloadAndExtractNewMod(
            activeGame,
            modDownloadUrl,
            modName,
            modSourceName,
          );
          if (!resp.success) {
            jobTracker.updateFailureReason(resp.msg);
            return false;
          }
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_decompile"),
      task: async () => {
        let resp = await decompileForModInstall(
          activeGame,
          modName,
          modSourceName,
        );
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_compile"),
      task: async () => {
        let resp = await compileForModInstall(
          activeGame,
          modName,
          modSourceName,
        );
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_done"),
      task: async () => {
        let resp = await saveModInstallInfo(
          activeGame,
          modName,
          modSourceName,
          modVersion,
        );
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    },
  ]);
}

export async function setupDecompileModJob(
  activeGame: SupportedGame,
  modName: string,
  modSourceName: string,
) {
  // Check to see if we need to prompt for the ISO or not
  jobTracker.init([
    {
      status: "queued",
      label: $format("setup_decompile"),
      task: async () => {
        let resp = await decompileForModInstall(
          activeGame,
          modName,
          modSourceName,
        );
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_done"),
      task: async () => {
        return true;
      },
    },
  ]);
}

export async function setupCompileModJob(
  activeGame: SupportedGame,
  modName: string,
  modSourceName: string,
) {
  // Check to see if we need to prompt for the ISO or not
  jobTracker.init([
    {
      status: "queued",
      label: $format("setup_compile"),
      task: async () => {
        let resp = await compileForModInstall(
          activeGame,
          modName,
          modSourceName,
        );
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_done"),
      task: async () => {
        return true;
      },
    },
  ]);
}
