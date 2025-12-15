<script lang="ts">
  import Requirements from "./Requirements.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Button } from "flowbite-svelte";
  import { folderPrompt, isoPrompt } from "$lib/utils/file-dialogs";
  import {
    isAVXRequirementMet,
    isDiskSpaceRequirementMet,
    isOpenGLRequirementMet,
    getProceedAfterSuccessfulOperation,
  } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { arch, type } from "@tauri-apps/plugin-os";
  import { isMinVCCRuntime, isMinMacOSVersion } from "$lib/stores/VersionStore";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { navigate } from "/src/router";
  import { asJobType } from "$lib/job/jobs";

  let { activeGame }: { activeGame: SupportedGame } = $props();

  let requirementsMet: boolean | undefined = $state(true);
  let proceedAfterSuccessfulOperation = $state(true);

  onMount(async () => {
    // Check requirements
    await checkRequirements();
    proceedAfterSuccessfulOperation =
      await getProceedAfterSuccessfulOperation();
  });

  async function checkRequirements() {
    const architecture = arch();
    const osType = type();
    const isOpenGLMet = await isOpenGLRequirementMet(false);
    const isDiskSpaceMet = await isDiskSpaceRequirementMet(activeGame);
    if (architecture === "aarch64") {
      // arm, we don't bother checking for simd
      // - if macOS (the only supported ARM platform), we check they are on atleast macOS 15
      // there is no easy way to check to see if they have rosetta 2, if you know of one, contribute it
      if (osType !== "macos") {
        requirementsMet = false;
        return;
      }
      requirementsMet = $isMinMacOSVersion && isOpenGLMet && isDiskSpaceMet;
    } else {
      const isAvxMet = await isAVXRequirementMet();
      if (osType == "windows") {
        requirementsMet =
          isAvxMet && isOpenGLMet && isDiskSpaceMet && $isMinVCCRuntime;
      } else {
        requirementsMet = isAvxMet && isOpenGLMet && isDiskSpaceMet;
      }
    }
  }

  async function install(viaFolder: boolean) {
    let sourcePath = undefined;
    if (viaFolder) {
      sourcePath = await folderPrompt($_("setup_prompt_selectFolderWithISO"));
    } else {
      sourcePath = await isoPrompt(
        $_("setup_prompt_ISOFileLabel"),
        $_("setup_prompt_selectISO"),
      );
    }
    if (sourcePath !== undefined) {
      navigate("/job/:job_type", {
        params: {
          job_type: asJobType("installGame"),
        },
        search: {
          activeGame: activeGame,
          sourcePath: sourcePath
        },
      });
    }
  }
</script>

{#if !requirementsMet}
  <Requirements {activeGame} on:recheckRequirements={checkRequirements} />
{:else}
  <div class="flex flex-col justify-end items-end mt-auto">
    <h1
      class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline"
    >
      {$_(`gameName_${activeGame}`)}
    </h1>
    <div class="flex flex-row gap-2">
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => await install(false)}
        >{$_("setup_button_installViaISO")}</Button
      >
      <!-- TODO - disabled for now, needs fixes in the extractor -->
      <!-- <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => await install(true)}
        >Install via Folder</Button
      > -->
    </div>
  </div>
{/if}
