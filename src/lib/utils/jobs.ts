import {
  runDecompiler,
  runCompiler,
  updateDataDirectory,
  extractAndValidateISO,
} from "$lib/rpc/binaries";
import { finalizeInstallation } from "$lib/rpc/config";
import {
  baseGameIsoExists,
  compileForModInstall,
  decompileForModInstall,
  downloadAndExtractNewMod,
  extractIsoForModInstall,
  saveModInstallInfo,
} from "$lib/rpc/features";
import { folderPrompt, isoPrompt } from "./file-dialogs";
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import { progressTracker } from "$lib/stores/ProgressStore";
import { invalidateAll } from "$app/navigation";
import { unwrapFunctionStore, format } from "svelte-i18n";

const $format = unwrapFunctionStore(format);

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
    label: $format("setup_decompile"),
    task: async () => runDecompiler("", game, true, true),
  },
  {
    label: $format("setup_done"),
    task: async () => ({ success: true }),
    callback: async () => progressTracker.clear(),
  },
];

export const compileJob = (game: SupportedGame): JobStep[] => [
  {
    label: $format("setup_compile"),
    task: () => runCompiler("", game, true),
  },
  {
    label: $format("setup_done"),
    task: async () => ({ success: true }),
    callback: async () => progressTracker.clear(),
  },
];

export const updateGameJob = (game: SupportedGame): JobStep[] => [
  {
    label: $format("setup_copyFiles"),
    task: () => updateDataDirectory(game),
  },
  {
    label: $format("setup_decompile"),
    task: () => runDecompiler("", game, true, false),
  },
  {
    label: $format("setup_compile"),
    task: () => runCompiler("", game),
  },
  {
    label: $format("setup_done"),
    task: async () => ({ success: true }),
    callback: async () => {
      await finalizeInstallation(game);
      progressTracker.clear();
      await invalidateAll();
    },
  },
];

export async function installBaseGame(
  game: SupportedGame,
  viaFolder: boolean = false,
) {
  const sourcePath = viaFolder
    ? await folderPrompt($format("setup_prompt_selectFolderWithISO"))
    : await isoPrompt(
        $format("setup_prompt_ISOFileLabel"),
        $format("setup_prompt_selectISO"),
      );

  if (!sourcePath) return;

  const steps: JobStep[] = [
    {
      label: $format("setup_extractAndVerify"),
      task: async () => {
        const resp = await extractAndValidateISO(sourcePath, game);
        return resp?.success
          ? resp
          : {
              success: false,
              msg: resp?.msg ?? $format("setup_error_extractVerifyFailed"),
            };
      },
    },
    {
      label: $format("setup_decompile"),
      task: async () => {
        const resp = await runDecompiler(sourcePath, game, false, false);
        return resp?.success
          ? resp
          : {
              success: false,
              msg: resp?.msg ?? $format("setup_error_decompileFailed"),
            };
      },
    },
    {
      label: $format("setup_compile"),
      task: async () => {
        const resp = await runCompiler(sourcePath, game);
        return resp?.success
          ? resp
          : {
              success: false,
              msg: resp?.msg ?? $format("setup_error_compileFailed"),
            };
      },
    },
    {
      label: $format("setup_done"),
      task: async () => {
        await finalizeInstallation(game);
        return { success: true };
      },
      callback: async () => {
        await invalidateAll();
      },
    },
  ];

  await runJob(steps);
}

export const installModExternal = (
  game,
  modName,
  modSourceName,
  modVersion,
  modDownloadUrl,
): JobStep[] => [
  {
    label: $format("setup_extractAndVerify"),
    task: async () => {
      const isoPresent = await baseGameIsoExists(game);
      if (!isoPresent) {
        const sourcePath = await isoPrompt(
          $format("setup_prompt_ISOFileLabel"),
          $format("setup_prompt_selectISO"),
        );
        if (!sourcePath) {
          return { success: false, msg: $format("setup_error_missingISO") };
        }
        const r = await extractIsoForModInstall(
          game,
          modName,
          modSourceName,
          sourcePath,
        );
        if (!r?.success)
          return {
            success: false,
            msg: r?.msg ?? $format("setup_error_extractISOFailed"),
          };
      }

      const d = await downloadAndExtractNewMod(
        game,
        modDownloadUrl,
        modName,
        modSourceName,
      );
      if (!d?.success)
        return {
          success: false,
          msg: d?.msg ?? $format("setup_error_downloadExtractFailed"),
        };

      return { success: true };
    },
  },
  {
    label: $format("setup_decompile"),
    task: async () => {
      const r = await decompileForModInstall(game, modName, modSourceName);
      return r?.success
        ? r
        : {
            success: false,
            msg: r?.msg ?? $format("setup_error_decompileFailed"),
          };
    },
  },
  {
    label: $format("setup_compile"),
    task: async () => {
      const r = await compileForModInstall(game, modName, modSourceName);
      return r?.success
        ? r
        : {
            success: false,
            msg: r?.msg ?? $format("setup_error_compileFailed"),
          };
    },
  },
  {
    label: $format("setup_done"),
    task: async () => {
      const r = await saveModInstallInfo(
        game,
        modName,
        modSourceName,
        modVersion,
      );
      return r?.success
        ? r
        : { success: false, msg: r?.msg ?? $format("setup_error_saveFailed") };
    },
    callback: async () => {
      progressTracker.clear();
      await invalidateAll();
    },
  },
];
