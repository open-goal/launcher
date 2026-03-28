import { writable, derived } from "svelte/store";
import { arch, type } from "@tauri-apps/plugin-os";
import {
  isAVXRequirementMet,
  isDiskSpaceRequirementMet,
  isOpenGLRequirementMet,
} from "$lib/rpc/config";
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import { systemInfoState } from "/src/state/SystemInfoState.svelte";

export interface RequirementResult {
  architecture: string;
  osType: string;

  isAVXRelevant: boolean;
  isTryingToUseARMOutsideOfMacOS: boolean;
  isVCCRelevant: boolean;

  isAVXMet?: boolean;
  isOpenGLMet?: boolean;
  isDiskSpaceMet?: boolean;

  requirementsMet: boolean;
}

async function evaluateRequirements(
  game: SupportedGame,
  forceOpenGL = false,
): Promise<RequirementResult> {
  const architecture = arch();
  const osType = type();

  const isAVXRelevant = osType !== "macos";
  const isTryingToUseARMOutsideOfMacOS =
    architecture === "aarch64" && osType !== "macos";
  const isVCCRelevant = osType === "windows";

  const isOpenGLMet = await isOpenGLRequirementMet(forceOpenGL);
  const isDiskSpaceMet = await isDiskSpaceRequirementMet(game);

  let isAVXMet: boolean | undefined;
  if (isAVXRelevant) {
    isAVXMet = await isAVXRequirementMet();
  }

  let requirementsMet: boolean;

  if (architecture === "aarch64" && osType !== "macos") {
    requirementsMet = false;
  } else if (osType === "macos") {
    requirementsMet = Boolean(isOpenGLMet && isDiskSpaceMet);
  } else if (osType === "windows") {
    requirementsMet = Boolean(
      isAVXMet &&
      isOpenGLMet &&
      isDiskSpaceMet &&
      systemInfoState.isMinVCCRuntimeInstalled,
    );
  } else {
    requirementsMet = Boolean(isAVXMet && isOpenGLMet && isDiskSpaceMet);
  }

  return {
    architecture,
    osType,
    isAVXRelevant,
    isTryingToUseARMOutsideOfMacOS,
    isVCCRelevant,
    isAVXMet,
    isOpenGLMet,
    isDiskSpaceMet,
    requirementsMet,
  };
}

interface ReqState {
  game?: SupportedGame;
  result?: RequirementResult;
}

const state = writable<ReqState>({});

async function refresh(game: SupportedGame, forceOpenGL = false) {
  const res = await evaluateRequirements(game, forceOpenGL);
  state.set({ game, result: res });
  return res;
}

export const requirementsStore = {
  subscribe: state.subscribe,
  refresh,
};

export const currentRequirements = derived(state, (s) => s.result);
export const currentGame = derived(state, (s) => s.game);
