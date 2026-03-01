<script lang="ts">
  import { Alert, Button } from "flowbite-svelte";
  import { setBypassRequirements } from "$lib/rpc/config";
  import {
    requirementsStore,
    currentRequirements,
  } from "/src/state/requirements-store";
  import { _ } from "svelte-i18n";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import { systemInfoState } from "/src/state/SystemInfoState.svelte";
  import { route } from "/src/router";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";

  const gameParam = $derived(route.params.game_name);
  const activeGame: SupportedGame | undefined = toSupportedGame(gameParam);

  function alertColor(val: boolean | undefined) {
    if (val === undefined) {
      return "yellow";
    }
    return val ? "green" : "red";
  }
</script>

{#if activeGame}
  <div
    class="flex flex-col h-full justify-center items-center p-5 text-center gap-3"
  >
    <h1 class="text-xl font-black mb-5 text-outline">
      {$_("requirements_notMet_header")}
    </h1>
    {#if $currentRequirements?.isAVXRelevant}
      {#if !$currentRequirements?.isAVXMet}
        <Alert
          class="w-full text-start"
          rounded={false}
          color={alertColor($currentRequirements?.isAVXMet)}
        >
          {#if $currentRequirements?.isAVXMet === undefined}
            <span class="font-bold"
              >{$_("requirements_cpu_unableToCheckAVX")}</span
            >
          {:else}
            <span class="font-bold"
              >{$_("requirements_cpu_doesNotSupportAVX")}</span
            >
            <ul class="font-medium list-disc list-inside">
              <li>{$_("requirements_cpu_avxExplanation_1")}</li>
              <li>{$_("requirements_cpu_avxExplanation_2")}</li>
              <li>
                <a
                  class="font-bold text-blue-500"
                  target="_blank"
                  rel="noreferrer"
                  href="https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX"
                  >{$_("requirements_cpu_avxExplanation_3")}</a
                >
              </li>
            </ul>
          {/if}
        </Alert>
      {/if}
    {:else if $currentRequirements?.isTryingToUseARMOutsideOfMacOS}
      <Alert
        class="w-full text-start"
        rounded={false}
        color={alertColor(
          !$currentRequirements?.isTryingToUseARMOutsideOfMacOS,
        )}
      >
        <span class="font-bold"
          >{$_("requirements_armNotSupportedOutsideMacOS")}</span
        >
      </Alert>
    {:else if !systemInfoState.isMinMacOSVersion}
      <Alert
        class="w-full text-start"
        rounded={false}
        color={alertColor(systemInfoState.isMinMacOSVersion)}
      >
        {#if systemInfoState.isMinMacOSVersion === undefined}
          <span class="font-bold"
            >{$_("requirements_macos_unableToCheckVersion")}</span
          >
        {:else}
          <span class="font-bold"
            >{$_("requirements_macos_notAtleastVersion15")}</span
          >
        {/if}
      </Alert>
    {/if}

    {#if !$currentRequirements?.isOpenGLMet}
      <Alert
        class="w-full text-start"
        rounded={false}
        color={alertColor($currentRequirements?.isOpenGLMet)}
      >
        {#if $currentRequirements?.isOpenGLMet === undefined}
          <span class="font-bold"
            >{$_("requirements_gpu_unableToCheckOpenGL")}</span
          >
        {:else}
          <span class="font-bold"
            >{$_("requirements_gpu_doesNotSupportOpenGL")}</span
          >
          <ul class="font-medium list-disc list-inside">
            <li>
              {$_("requirements_gpu_avxExplanation_1_preLink")}
              <a
                class="font-bold text-blue-500"
                target="_blank"
                rel="noreferrer"
                href="https://www.techpowerup.com/gpu-specs/"
                >{$_("requirements_gpu_avxExplanation_1_link")}</a
              >
              {$_("requirements_gpu_avxExplanation_1_postLink")}
            </li>
            <li>{$_("requirements_gpu_avxExplanation_2")}</li>
            <li>
              {$_("requirements_gpu_avxExplanation_3")}
            </li>
          </ul>
        {/if}
      </Alert>
    {/if}

    {#if !$currentRequirements?.isDiskSpaceMet}
      <Alert
        class="w-full text-start"
        rounded={false}
        color={alertColor($currentRequirements?.isDiskSpaceMet)}
      >
        {#if $currentRequirements?.isDiskSpaceMet === undefined}
          <span class="font-bold"
            >{$_(`requirements_disk_unableToCheckSpace`)}</span
          >
        {:else}
          <span class="font-bold"
            >{$_(`requirements_disk_notEnoughSpace_${activeGame}`)}</span
          >
        {/if}
      </Alert>
    {/if}

    {#if $currentRequirements?.isVCCRelevant}
      {#if !systemInfoState.isMinVCCRuntimeInstalled}
        <Alert
          class="w-full text-start"
          rounded={false}
          color={alertColor(systemInfoState.isMinVCCRuntimeInstalled)}
        >
          {#if systemInfoState.isMinVCCRuntimeInstalled === undefined}
            <span class="font-bold"
              >{$_("requirements_windows_cantCheckIfVccRuntimeInstalled")}</span
            >
          {:else}
            <span class="font-bold"
              >{$_("requirements_windows_vccRuntimeNotInstalled")}</span
            >
            <ul class="font-medium list-disc list-inside">
              <li>{$_("requirements_windows_vccRuntimeExplanation")}</li>
              <li>
                <a
                  class="font-bold text-blue-500"
                  target="_blank"
                  rel="noreferrer"
                  href="https://aka.ms/vs/17/release/vc_redist.x64.exe"
                  >{$_(
                    "requirements_windows_vccRuntimeExplanation_downloadLink",
                  )}</a
                >
              </li>
            </ul>
          {/if}
        </Alert>
      {/if}
    {/if}
    <div>
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          await requirementsStore.refresh(activeGame, true);
        }}>{$_("requirements_button_recheck")}</Button
      >
      <Button
        hidden={$currentRequirements?.isVCCRelevant &&
          !systemInfoState.isMinVCCRuntimeInstalled}
        class="border-solid border-2 border-slate-900 rounded bg-orange-800 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          const confirmed = await confirm(
            `${$_("requirements_button_bypass_warning_1")}\n\n${$_(
              "requirements_button_bypass_warning_2",
            )}`,
            { title: "OpenGOAL Launcher", kind: "warning" },
          );
          if (confirmed) {
            await setBypassRequirements(true);
            await requirementsStore.refresh(activeGame, false);
          }
        }}>{$_("requirements_button_bypass")}</Button
      >
    </div>
  </div>
{/if}
