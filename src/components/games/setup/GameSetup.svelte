<script lang="ts">
  import Progress from "./Progress.svelte";
  import LogViewer from "./LogViewer.svelte";
  import Requirements from "./Requirements.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import {
    extractAndValidateISO,
    runCompiler,
    runDecompiler,
  } from "$lib/rpc/binaries";
  import { folderPrompt, isoPrompt } from "$lib/utils/file-dialogs";
  import {
    finalizeInstallation,
    isAVXRequirementMet,
    isDiskSpaceRequirementMet,
    isOpenGLRequirementMet,
    getProceedAfterSuccessfulOperation,
  } from "$lib/rpc/config";
  import { progressTracker } from "$lib/stores/ProgressStore";
  import { generateSupportPackage } from "$lib/rpc/support";
  import { _ } from "svelte-i18n";
  import { emit } from "@tauri-apps/api/event";
  import { arch, type } from "@tauri-apps/plugin-os";
  import { isMinVCCRuntime, isMinMacOSVersion } from "$lib/stores/VersionStore";
  import { activeGame } from "$lib/stores/AppStore";

  const dispatch = createEventDispatcher();

  let requirementsMet: boolean | undefined = true;
  let installing = false;
  let installationError = undefined;
  let proceedAfterSuccessfulOperation = true;

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
    const isDiskSpaceMet = await isDiskSpaceRequirementMet($activeGame);
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
    let sourcePath = "";
    if (viaFolder) {
      sourcePath = await folderPrompt($_("setup_prompt_selectFolderWithISO"));
    } else {
      sourcePath = await isoPrompt(
        $_("setup_prompt_ISOFileLabel"),
        $_("setup_prompt_selectISO"),
      );
    }
    if (sourcePath !== undefined) {
      installing = true;
      installationError = undefined;
      // Initialize the installation steps for this particular config
      progressTracker.init([
        {
          status: "queued",
          label: $_("setup_extractAndVerify"),
        },
        {
          status: "queued",
          label: $_("setup_decompile"),
        },
        {
          status: "queued",
          label: $_("setup_compile"),
        },
        {
          status: "queued",
          label: $_("setup_done"),
        },
      ]);
      // TODO - make this cleaner
      progressTracker.start();
      let resp = await extractAndValidateISO(sourcePath, $activeGame);
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      resp = await runDecompiler(sourcePath, $activeGame, false, false);
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      resp = await runCompiler(sourcePath, $activeGame);
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      // TODO - technically should handle the error here too
      await finalizeInstallation($activeGame);
      await emit("gameInstalled");
      progressTracker.proceed();
    }
  }

  $: if (
    $progressTracker.overallStatus === "success" &&
    proceedAfterSuccessfulOperation
  ) {
    dispatch("change");
  }
</script>

{#if !requirementsMet}
  <Requirements on:recheckRequirements={checkRequirements} />
{:else if installing}
  <div class="flex flex-col justify-content shrink">
    <Progress />
    <LogViewer />
  </div>
  {#if $progressTracker.overallStatus === "success" && !proceedAfterSuccessfulOperation}
    <div class="flex flex-col justify-end items-end mt-auto">
      <div class="flex flex-row gap-2">
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          on:click={async () => dispatch("change")}
          >{$_("setup_button_continue")}</Button
        >
      </div>
    </div>
  {:else if $progressTracker.overallStatus === "failed"}
    <div class="flex flex-col mt-auto">
      <div class="flex flex-row gap-2">
        <Alert
          color="red"
          class="dark:bg-slate-900 flex-grow border-t-4"
          rounded={false}
        >
          <span class="font-medium text-red-500"
            >{$_("setup_installationFailed")}
          </span><span class="text-white"> {installationError}</span>
        </Alert>
        <!-- TODO - no button to go back -->
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          on:click={async () => await generateSupportPackage()}
          >{$_("setup_button_getSupportPackage")}</Button
        >
      </div>
    </div>
  {/if}
{:else}
  <div class="flex flex-col justify-end items-end mt-auto">
    <h1
      class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline"
    >
      {$_(`gameName_${$activeGame}`)}
    </h1>
    <div class="flex flex-row gap-2">
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => await install(false)}
        >{$_("setup_button_installViaISO")}</Button
      >
      <!-- TODO - disabled for now, needs fixes in the extractor -->
      <!-- <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => await install(true)}
        >Install via Folder</Button
      > -->
    </div>
  </div>
{/if}
