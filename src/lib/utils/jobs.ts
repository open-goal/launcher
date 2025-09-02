import {
  runDecompiler,
  runCompiler,
  updateDataDirectory,
} from "$lib/rpc/binaries";
import { finalizeInstallation } from "$lib/rpc/config";
import { baseGameIsoExists } from "$lib/rpc/features";
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
