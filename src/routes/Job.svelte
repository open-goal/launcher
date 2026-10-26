<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { Button, Spinner, Tooltip } from "flowbite-svelte";
  import IconAlertCircle from "~icons/mdi/alert-circle-outline";
  import IconCheckCircle from "~icons/mdi/check-circle";
  import IconDiscord from "~icons/mdi/discord";
  import IconFolderOpen from "~icons/mdi/folder-open";
  import IconPackageDown from "~icons/mdi/package-down";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import { jobTracker } from "$lib/stores/JobStore";
  import { toastStore } from "$lib/stores/ToastStore";
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
  import { toSupportedGame } from "$lib/rpc/SupportedGame";
  import { setupTexturePacks } from "$lib/job/texturePackJob";
  import { infoLog } from "$lib/rpc/logging";
  import { config } from "../state/config.svelte";

  const proceedAfterSuccess = config?.proceedAfterSuccessfulOperation;

  let invalidJobDefinition = $state(false);
  let generatingPackage = $state(false);
  let packagePath: string | undefined = $state(undefined);

  async function getSupportPackage() {
    generatingPackage = true;
    const path = await generateSupportPackage();
    generatingPackage = false;
    if (path) {
      packagePath = path;
      toastStore.makeToast($_("setup_supportPackage_saved"), "info");
      await revealItemInDir(path);
    }
  }

  onMount(async () => {
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

    // - activeGame
    if (searchParams.has("activeGame")) {
      activeGame = toSupportedGame(searchParams.get("activeGame")?.toString());
    }
    // - sourcePath
    if (searchParams.has("sourcePath")) {
      sourcePath = searchParams.get("sourcePath")?.toString() ?? undefined;
    }
    // - packsToDelete
    if (searchParams.has("packsToDelete")) {
      packsToDelete = JSON.parse(
        searchParams.get("packsToDelete")?.toString() ?? "",
      );
    }
    // - enabledPacks
    if (searchParams.has("enabledPacks")) {
      enabledPacks = JSON.parse(
        searchParams.get("enabledPacks")?.toString() ?? "",
      );
    }
    // - modName
    if (searchParams.has("modName")) {
      modName = searchParams.get("modName")?.toString() ?? undefined;
    }
    // - modSourceName
    if (searchParams.has("modSourceName")) {
      modSourceName =
        searchParams.get("modSourceName")?.toString() ?? undefined;
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

    infoLog(`Initializing job type: ${jobType}`);

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
          modSourceName &&
          modDownloadUrl &&
          modVersion
        ) {
          await setupModInstallation(
            activeGame,
            modName,
            modSourceName,
            modDownloadUrl,
            modVersion,
          );
        }
        break;
      case "installModLocally":
        if (activeGame && modName && modSourceName) {
          await setupModInstallation(
            activeGame,
            modName,
            modSourceName,
            undefined,
            "local",
          );
        }
        break;
      case "decompileMod":
        if (activeGame && modName && modSourceName) {
          await setupDecompileModJob(activeGame, modName, modSourceName);
        } else {
          invalidJobDefinition = true;
        }
        break;
      case "compileMod":
        if (activeGame && modName && modSourceName) {
          await setupCompileModJob(activeGame, modName, modSourceName);
        } else {
          invalidJobDefinition = true;
        }
        break;
      default:
        console.error("Unhandled job type", jobType);
        invalidJobDefinition = true;
    }
    if (!invalidJobDefinition) {
      jobTracker.start();
    }
  });

  onDestroy(() => {
    jobTracker.clear();
  });

  $effect(() => {
    if ($jobTracker.overallStatus === "success" && proceedAfterSuccess) {
      jobTracker.clear();
      const returnTo = searchParams.get("returnTo")?.toString();
      if (returnTo) {
        navigate(returnTo as any);
      } else {
        navigate(-1);
      }
    }
  });
</script>

<div class="flex flex-col h-full p-5">
  {#if invalidJobDefinition}
    App got into a bad state, report this to the devs!
  {:else}
    <div class="flex flex-col justify-content">
      <Progress />
      <LogViewer />
    </div>
    {#if $jobTracker.overallStatus === "success" && !proceedAfterSuccess}
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
      <div class="flex flex-col gap-3 mt-auto p-4 rounded bg-slate-900">
        <div class="flex flex-row items-start gap-3">
          <IconAlertCircle class="text-red-500 text-2xl shrink-0" />
          <div class="flex flex-col">
            <span class="font-medium text-red-500"
              >{$_("setup_installationFailed")}</span
            >
            <span class="text-sm text-white">{$jobTracker.failureReason}</span>
          </div>
        </div>
        <div class="flex flex-row items-center gap-4 p-3 rounded bg-slate-800">
          <div class="flex flex-col gap-2 grow">
            <span class="text-sm font-semibold"
              >{packagePath
                ? $_("setup_supportPackage_headerSaved")
                : $_("setup_supportPackage_header")}</span
            >
            <ol class="grid grid-cols-3 gap-3 text-xs">
              <li class="flex flex-row gap-2">
                {#if packagePath}
                  <IconCheckCircle class="text-green-500 text-xl shrink-0" />
                  <div class="flex flex-col">
                    <span class="font-semibold text-green-500"
                      >{$_("setup_supportPackage_step1_saved")}</span
                    >
                    <span class="text-slate-400 break-all"
                      >{packagePath.split(/[\\/]/).pop()}</span
                    >
                  </div>
                {:else}
                  <span
                    class="flex items-center justify-center shrink-0 w-5 h-5 rounded-full bg-orange-400 text-slate-900 font-semibold"
                    >1</span
                  >
                  <div class="flex flex-col">
                    <span class="font-semibold"
                      >{$_("setup_supportPackage_step1")}</span
                    >
                    <span class="text-slate-400"
                      >{$_("setup_supportPackage_step1_desc")}</span
                    >
                  </div>
                {/if}
              </li>
              {#each [2, 3] as step}
                <li class="flex flex-row gap-2">
                  <span
                    class="flex items-center justify-center shrink-0 w-5 h-5 rounded-full bg-slate-700 font-semibold"
                    >{step}</span
                  >
                  <div class="flex flex-col">
                    <span class="font-semibold"
                      >{$_(`setup_supportPackage_step${step}`)}</span
                    >
                    <span class="text-slate-400"
                      >{$_(`setup_supportPackage_step${step}_desc`)}</span
                    >
                  </div>
                </li>
              {/each}
            </ol>
          </div>
          <div class="flex flex-col gap-2 shrink-0">
            {#if packagePath}
              <Button
                class="border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
                href="https://discord.gg/dPRCfsju3N"
                target="_blank"
                rel="noreferrer noopener"
                ><IconDiscord />&nbsp;{$_(
                  "setup_button_openDiscordHelp",
                )}</Button
              >
              <Button
                class="border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
                onclick={() => packagePath && revealItemInDir(packagePath)}
                ><IconFolderOpen />&nbsp;{$_(
                  "setup_button_showInFolder",
                )}</Button
              >
            {:else}
              <Button
                id="get-support-package"
                disabled={generatingPackage}
                class="border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
                onclick={getSupportPackage}
              >
                {#if generatingPackage}
                  <Spinner class="mr-3" size="4" color="yellow" />
                  {$_("setup_button_collectingLogs")}
                {:else}
                  <IconPackageDown />&nbsp;{$_(
                    "setup_button_getSupportPackage",
                  )}
                {/if}
              </Button>
              <Tooltip
                triggeredBy="#get-support-package"
                placement="top"
                type="dark"
                class="max-w-64"
              >
                <p class="font-semibold">
                  {$_("setup_supportPackage_tooltip_header")}
                </p>
                <p class="text-xs">{$_("setup_supportPackage_tooltip_body")}</p>
              </Tooltip>
              <Button
                class="border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
                href="https://discord.gg/dPRCfsju3N"
                target="_blank"
                rel="noreferrer noopener"
                ><IconDiscord />&nbsp;{$_(
                  "setup_button_openDiscordHelp",
                )}</Button
              >
            {/if}
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>
