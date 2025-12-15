<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import { jobTracker } from "$lib/stores/JobStore";
  import { getProceedAfterSuccessfulOperation } from "$lib/rpc/config";
  import { generateSupportPackage } from "$lib/rpc/support";
  import { _ } from "svelte-i18n";
  import { navigate, route } from "../router";
  import { asJobType, toJobType } from "$lib/job/jobs";
  import {
    setupCompileJob,
    setupDecompileJob,
    setupInstallGame,
    setupUpdateGameJob,
  } from "$lib/job/gameJob";
  import {
    setupModInstallation,
    setupDecompileModJob,
    setupCompileModJob,
  } from "$lib/job/modJob";
  import LogViewer from "../components/job/LogViewer.svelte";
  import Progress from "../components/job/Progress.svelte";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { searchParams } from "sv-router";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import { setupTexturePacks } from "$lib/job/texturePackJob";

  let proceedAfterSuccessfulOperation = $state(true);
  // TODO NOW - wire this up
  let invalidJobDefinition = $state(false);

  onMount(async () => {
    proceedAfterSuccessfulOperation =
      await getProceedAfterSuccessfulOperation();

    const jobType = toJobType(route.params.job_type);
    if (!jobType) {
      invalidJobDefinition = true;
      return;
    }

    // Some jobs require some args
    let activeGame: SupportedGame | undefined = undefined;
    let sourcePath: string | undefined = undefined;
    let packsToDelete: string[] = [];
    let enabledPacks: string[] = [];
    let modName: string | undefined = undefined;
    let modSourceName: string | undefined = undefined;
    let modDownloadUrl: string | undefined = undefined;
    let modVersion: string | undefined = undefined;

    // TODO NOW - move validation down to switch
    // - activeGame
    if (
      [
        asJobType("installGame"),
        asJobType("decompile"),
        asJobType("compile"),
        asJobType("updateGame"),
      ].includes(jobType)
    ) {
      if (!searchParams.has("activeGame")) {
        invalidJobDefinition = true;
        return;
      }
      activeGame = toSupportedGame(searchParams.get("activeGame")?.toString());
      if (!activeGame) {
        invalidJobDefinition = true;
        return;
      }
    }
    // - sourcePath
    if ([asJobType("installGame")].includes(jobType)) {
      if (!searchParams.has("sourcePath")) {
        invalidJobDefinition = true;
        return;
      }
      sourcePath = searchParams.get("sourcePath")?.toString();
      if (!sourcePath) {
        invalidJobDefinition = true;
        return;
      }
    }
    // - packsToDelete
    if (searchParams.has("packsToDelete")) {
      packsToDelete = (
        searchParams.get("packsToDelete")?.toString() ?? ""
      ).split(",");
    }
    // - enabledPacks
    if (searchParams.has("enabledPacks")) {
      enabledPacks = (searchParams.get("enabledPacks")?.toString() ?? "").split(
        ",",
      );
    }
    // - modName
    if (searchParams.has("modName")) {
      modName = searchParams.get("modName")?.toString() ?? undefined;
    }
    // - modSourceName
    if (searchParams.has("modSourceName")) {
      modSourceName = searchParams.get("modSourceName")?.toString() ?? undefined;
    }
    // - modDownloadUrl
    if (searchParams.has("modDownloadUrl")) {
      modDownloadUrl =
        searchParams.get("modDownloadUrl")?.toString() ?? undefined;
    }
    // - modVersion
    if (searchParams.has("modVersion")) {
      modVersion = searchParams.get("modVersion")?.toString() ?? undefined;
    }

    switch (jobType) {
      case "installGame":
        if (activeGame && sourcePath) {
          await setupInstallGame(activeGame, sourcePath);
        }
        break;
      case "decompile":
        if (activeGame) {
          await setupDecompileJob(activeGame);
        }
        break;
      case "compile":
        if (activeGame) {
          await setupCompileJob(activeGame);
        }
        break;
      case "updateGame":
        if (activeGame) {
          await setupUpdateGameJob(activeGame);
        }
        break;
      case "applyTexturePacks":
        if (activeGame) {
          await setupTexturePacks(activeGame, packsToDelete, enabledPacks);
        }
        break;
      case "installModFromUrl":
        if (
          activeGame &&
          modName &&
          sourceName &&
          modDownloadUrl &&
          modVersion
        ) {
          await setupModInstallation(
            activeGame,
            modName,
            sourceName,
            modDownloadUrl,
            modVersion,
          );
        }
        break;
      case "installModLocally":
        if (activeGame && modName && sourceName) {
          await setupModInstallation(
            activeGame,
            modName,
            sourceName,
            undefined,
            "local",
          );
        }
        break;
      case "decompileMod":
        if (activeGame && modName && sourceName) {
          await setupDecompileModJob(activeGame, modName, sourceName);
        }
        break;
      case "compileMod":
        if (activeGame && modName && sourceName) {
          await setupCompileModJob(activeGame, modName, sourceName);
        }
        break;
      default:
        console.error("Unhandled job type", jobType);
        invalidJobDefinition = true;
    }
    jobTracker.start();
  });

  onDestroy(() => {
    jobTracker.clear();
  });

  $effect(() => {
    if (
      $jobTracker.overallStatus === "success" &&
      proceedAfterSuccessfulOperation
    ) {
      jobTracker.clear();
      navigate(-1);
    }
  });
</script>

<div class="flex flex-col h-full p-5">
  <div class="flex flex-col justify-content">
    <Progress />
    <LogViewer />
  </div>
  {#if $jobTracker.overallStatus === "success" && !proceedAfterSuccessfulOperation}
    <div class="flex flex-col justify-end items-end mt-auto">
      <div class="flex flex-row gap-2">
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          onclick={() => {
            navigate(-1);
          }}>{$_("setup_button_continue")}</Button
        >
      </div>
    </div>
  {:else if $jobTracker.overallStatus === "failed"}
    <div class="flex flex-col mt-auto">
      <div class="flex flex-row gap-2">
        <Alert class="dark:bg-slate-900 flex-grow text-red-400">
          <span class="font-medium text-red-500"
            >{$_("setup_installationFailed")}
          </span><span class="text-white"> {$jobTracker.failureReason}</span>
        </Alert>
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          onclick={async () => await generateSupportPackage()}
          >{$_("setup_button_getSupportPackage")}</Button
        >
      </div>
    </div>
  {/if}
</div>
