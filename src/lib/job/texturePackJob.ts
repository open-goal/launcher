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
        let error = await deleteTexturePacks(activeGame, packsToDelete);
        if (error) {
          jobTracker.updateFailureReason(error);
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
        let error = await setEnabledTexturePacks(activeGame, enabledPacks);
        if (error) {
          jobTracker.updateFailureReason(error);
          return false;
        }
        return true;
      },
    },
    {
      status: "queued",
      label: $format("gameJob_applyTexturePacks"),
      task: async () => {
        let error = await updateTexturePackData(activeGame);
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
  );
  jobTracker.init(jobs);
}
