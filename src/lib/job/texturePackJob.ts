import { runDecompiler } from "$lib/rpc/binaries";
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import { setEnabledTexturePacks } from "$lib/rpc/config";
import { deleteTexturePacks, updateTexturePackData } from "$lib/rpc/features";
import { jobTracker, type JobStep } from "$lib/stores/JobStore";
import { unwrapFunctionStore, format } from "svelte-i18n";

const $format = unwrapFunctionStore(format);

export async function setupTexturePacks(
  activeGame: SupportedGame,
  packsToDelete: string[],
  enabledPacks: string[],
) {
  let jobs: JobStep[] = [];
  if (packsToDelete.length) {
    jobs.push({
      status: "queued",
      label: $format("gameJob_deleteTexturePacks"),
      task: async () => {
        let resp = await deleteTexturePacks(activeGame, packsToDelete);
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    });
  }
  jobs.push(
    {
      status: "queued",
      label: $format("gameJob_enablingTexturePacks"),
      task: async () => {
        let resp = await setEnabledTexturePacks(activeGame, enabledPacks);
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("gameJob_applyTexturePacks"),
      task: async () => {
        let resp = await updateTexturePackData(activeGame);
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("setup_decompile"),
      task: async () => {
        let resp = await runDecompiler("", activeGame, true, false);
        if (!resp.success) {
          jobTracker.updateFailureReason(resp.msg);
          return false;
        }
        return true;
      },
    },
  );
  jobTracker.init(jobs);
}
