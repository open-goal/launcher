import {
  runDecompiler,
  runCompiler,
  updateDataDirectory,
} from "$lib/rpc/binaries";
import { finalizeInstallation } from "$lib/rpc/config";
import { baseGameIsoExists, compileForModInstall, decompileForModInstall, downloadAndExtractNewMod, extractIsoForModInstall, saveModInstallInfo } from "$lib/rpc/features";
import { isoPrompt } from "./file-dialogs";
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import { progressTracker } from "$lib/stores/ProgressStore";
import { invalidateAll } from "$app/navigation";

export interface JobStep {
  label: string;
  task: () => Promise<{ success: boolean; msg?: string }>;
  callback?: () => void | Promise<void>;
}

export async function runJob(steps: JobStep[]) {
  progressTracker.init(
    steps.map(({ label }) => ({
      status: "queued",
      label,
    })),
  );
  progressTracker.start();

  for (const { task, callback } of steps) {
    const resp = await task();
    if (!resp.success) {
      progressTracker.fail(resp?.msg);
      return;
    }
    progressTracker.proceed();
    await callback?.();
  }
}

export const decompileJob = (game: SupportedGame): JobStep[] => [
  {
    label: "setup_decompile",
    task: async () => runDecompiler("", game, true, true),
  },
  {
    label: "setup_done",
    task: async () => ({ success: true }),
    callback: async () => progressTracker.clear(),
  },
];

export const compileJob = (game: SupportedGame): JobStep[] => [
  {
    label: "setup_compile",
    task: () => runCompiler("", game, true),
  },
  {
    label: "setup_done",
    task: async () => ({ success: true }),
    callback: async () => progressTracker.clear(),
  },
];

export const updateGameJob = (game: SupportedGame): JobStep[] => [
  {
    label: "setup_copyFiles",
    task: () => updateDataDirectory(game),
  },
  {
    label: "setup_decompile",
    task: () => runDecompiler("", game, true, false),
  },
  {
    label: "setup_compile",
    task: () => runCompiler("", game),
  },
  {
    label: "setup_done",
    task: async () => ({ success: true }),
    callback: async () => {
      await finalizeInstallation(game);
      progressTracker.clear();
      await invalidateAll();
    },
  },
];

export const installModExternal = (game, modName, modSourceName, modVersion, modDownloadUrl): JobStep[] => [
  {
    label: "setup_extractAndVerify",
    task: async () => {
      // 1) Ensure ISO extracted (prompt if missing)
      const isoPresent = await baseGameIsoExists(game);
      if (!isoPresent) {
        const sourcePath = await isoPrompt("setup_prompt_ISOFileLabel", "setup_prompt_selectISO");
        if (!sourcePath) {
          // Return a translated-friendly key; ProgressStore.fail(msg) will surface it
          return { success: false, msg: "setup_error_missingISO" };
        }
        const r = await extractIsoForModInstall(game, modName, modSourceName, sourcePath);
        if (!r?.success) return { success: false, msg: r?.msg ?? "setup_error_extractISOFailed" };
      }

      // 2) Download & extract the mod package
      const d = await downloadAndExtractNewMod(game, modDownloadUrl, modName, modSourceName);
      if (!d?.success) return { success: false, msg: d?.msg ?? "setup_error_downloadExtractFailed" };

      return { success: true };
    }
  },
  {
    label: "setup_decompile",
    task: async () => {
      const r = await decompileForModInstall(game, modName, modSourceName);
      return r?.success ? r : { success: false, msg: r?.msg ?? "setup_error_decompileFailed" };
    }
  },
  {
    label: "setup_compile",
    task: async () => {
      const r = await compileForModInstall(game, modName, modSourceName);
      return r?.success ? r : { success: false, msg: r?.msg ?? "setup_error_compileFailed" };
    }
  },
  {
    label: "setup_done",
    task: async () => {
      const r = await saveModInstallInfo(game, modName, modSourceName, modVersion);
      return r?.success ? r : { success: false, msg: r?.msg ?? "setup_error_saveFailed" };
    },
    callback: async () => {
      // optional: refresh any pages reading mod lists/config
      progressTracker.clear();
      await invalidateAll();
    }
  }
];