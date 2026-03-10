import {
  extractAndValidateISO,
  runDecompiler,
  runCompiler,
  updateDataDirectory,
} from "$lib/rpc/binaries";
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import { finalizeInstallation } from "$lib/rpc/config";
import { jobTracker } from "$lib/stores/JobStore";
import { emit } from "@tauri-apps/api/event";
import { unwrapFunctionStore, format } from "svelte-i18n";

const $format = unwrapFunctionStore(format);

export async function setupInstallGame(
  activeGame: SupportedGame,
  sourcePath: string,
) {
  // Initialize the installation steps for this particular config
  jobTracker.init([
    {
      status: "queued",
      label: $format("setup_extractAndVerify"),
      task: async () => {
        const error = await extractAndValidateISO(sourcePath, activeGame);
        if (error) {
          jobTracker.updateFailureReason(error);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_decompile"),
      task: async () => {
        const error = await runDecompiler(sourcePath, activeGame, false, false);
        if (error) {
          jobTracker.updateFailureReason(error);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_compile"),
      task: async () => {
        const error = await runCompiler(sourcePath, activeGame);
        if (error) {
          jobTracker.updateFailureReason(error);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_done"),
      task: async () => {
        try {
          await finalizeInstallation(activeGame);
          await emit("gameInstalled");
          return true;
        } catch (e) {
          return false;
        }
      },
    },
  ]);
}

export async function setupDecompileJob(activeGame: SupportedGame) {
  jobTracker.init([
    {
      status: "queued",
      label: $format("setup_decompile"),
      task: async () => {
        const error = await runDecompiler(null, activeGame, true, true);
        if (error) {
          jobTracker.updateFailureReason(error);
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

export async function setupCompileJob(activeGame: SupportedGame) {
  jobTracker.init([
    {
      status: "queued",
      label: $format("setup_compile"),
      task: async () => {
        const error = await runCompiler(null, activeGame, true);
        if (error) {
          jobTracker.updateFailureReason(error);
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

export async function setupUpdateGameJob(activeGame: SupportedGame) {
  jobTracker.init([
    {
      status: "queued",
      label: $format("setup_copyFiles"),
      task: async () => {
        let error = await updateDataDirectory(activeGame);
        if (error) {
          jobTracker.updateFailureReason(error);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_decompile"),
      task: async () => {
        let error = await runDecompiler(null, activeGame, true, false);
        if (error) {
          jobTracker.updateFailureReason(error);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_compile"),
      task: async () => {
        let error = await runCompiler(null, activeGame);
        if (error) {
          jobTracker.updateFailureReason(error);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_done"),
      task: async () => {
        try {
          await finalizeInstallation(activeGame);
          await emit("gameInstalled");
          return true;
        } catch (e) {
          return false;
        }
      },
    },
  ]);
}
