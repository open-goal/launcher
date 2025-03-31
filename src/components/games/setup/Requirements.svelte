<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import {
    isAVXRequirementMet,
    isDiskSpaceRequirementMet,
    isOpenGLRequirementMet,
    isMinimumVCCRuntimeInstalled,
    setBypassRequirements,
  } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import { getInternalName, type SupportedGame } from "$lib/constants";
  import { arch, type } from "@tauri-apps/plugin-os";
  import { isMacOSVersion15OrAbove } from "$lib/rpc/util";

  export let activeGame: SupportedGame;

  let isAVXRelevant = true;
  let isTryingToUseARMOutsideOfMacOS: boolean | undefined = false;
  let isMacOSVersionSufficient: boolean | undefined = false;
  let isAVXMet: boolean | undefined = false;
  let isOpenGLMet: boolean | undefined = false;
  let isDiskSpaceMet: boolean | undefined = false;
  let isVCCRelevant = false;
  let isVCCInstalled: boolean | undefined = false;

  const dispatch = createEventDispatcher();

  onMount(async () => {
    const architecture = await arch();
    const osType = await type();
    isOpenGLMet = await isOpenGLRequirementMet(false);
    isDiskSpaceMet = await isDiskSpaceRequirementMet(
      getInternalName(activeGame),
    );
    if (architecture === "aarch64") {
      isAVXRelevant = false;
      // arm, we don't bother checking for simd
      // - if macOS (the only supported ARM platform), we check they are on atleast macOS 15
      // there is no easy way to check to see if they have rosetta 2, if you know of one, contribute it
      if (osType !== "macos") {
        isTryingToUseARMOutsideOfMacOS = true;
      } else {
        isMacOSVersionSufficient = await isMacOSVersion15OrAbove();
      }
    } else {
      isAVXMet = await isAVXRequirementMet();
      if (osType == "windows") {
        isVCCRelevant = true;
        isVCCInstalled = await isMinimumVCCRuntimeInstalled();
      }
    }
  });

  function alertColor(val: boolean | undefined) {
    if (val === undefined) {
      return "yellow";
    }
    return val ? "green" : "red";
  }
</script>

<!-- TODO - good spot for a new component -->

<div
  class="flex flex-col h-full justify-center items-center p-5 text-center gap-3"
>
  <h1 class="text-xl font-black mb-5 text-outline">
    {$_("requirements_notMet_header")}
  </h1>
  {#if isAVXRelevant}
    <Alert
      class="w-full text-start"
      rounded={false}
      color={alertColor(isAVXMet)}
    >
      {#if isAVXMet}
        <span class="font-bold">{$_("requirements_cpu_supportsAVX")}</span>
      {:else if isAVXMet === undefined}
        <span class="font-bold">{$_("requirements_cpu_unableToCheckAVX")}</span>
      {:else}
        <span class="font-bold">{$_("requirements_cpu_doesNotSupportAVX")}</span
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
  {:else if isTryingToUseARMOutsideOfMacOS}
    <Alert
      class="w-full text-start"
      rounded={false}
      color={alertColor(!isTryingToUseARMOutsideOfMacOS)}
    >
      <span class="font-bold"
        >{$_("requirements_armNotSupportedOutsideMacOS")}</span
      >
    </Alert>
  {:else}
    <Alert
      class="w-full text-start"
      rounded={false}
      color={alertColor(isMacOSVersionSufficient)}
    >
      {#if isMacOSVersionSufficient}
        <span class="font-bold"
          >{$_("requirements_macos_atleastVersion15")}</span
        >
      {:else if isMacOSVersionSufficient === undefined}
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
  <Alert
    class="w-full text-start"
    rounded={false}
    color={alertColor(isOpenGLMet)}
  >
    {#if isOpenGLMet}
      <span class="font-bold">{$_("requirements_gpu_supportsOpenGL")}</span>
    {:else if isOpenGLMet === undefined}
      <span class="font-bold">{$_("requirements_gpu_unableToCheckOpenGL")}</span
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
  <Alert
    class="w-full text-start"
    rounded={false}
    color={alertColor(isDiskSpaceMet)}
  >
    {#if isDiskSpaceMet}
      <span class="font-bold"
        >{$_(
          `requirements_disk_enoughSpace_${getInternalName(activeGame)}`,
        )}</span
      >
    {:else if isDiskSpaceMet === undefined}
      <span class="font-bold">{$_(`requirements_disk_unableToCheckSpace`)}</span
      >
    {:else}
      <span class="font-bold"
        >{$_(
          `requirements_disk_notEnoughSpace_${getInternalName(activeGame)}`,
        )}</span
      >
    {/if}
  </Alert>
  {#if isVCCRelevant}
    <Alert
      class="w-full text-start"
      rounded={false}
      color={alertColor(isVCCInstalled)}
    >
      {#if isVCCInstalled}
        <span class="font-bold"
          >{$_("requirements_windows_vccRuntimeInstalled")}</span
        >
      {:else if isVCCInstalled === undefined}
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
  <div>
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={async () => {
        isAVXMet = await isAVXRequirementMet();
        isOpenGLMet = await isOpenGLRequirementMet(true);
        isMacOSVersionSufficient = await isMacOSVersion15OrAbove();
        dispatch("recheckRequirements");
      }}>{$_("requirements_button_recheck")}</Button
    >
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-orange-800 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={async () => {
        const confirmed = await confirm(
          `${$_("requirements_button_bypass_warning_1")}\n\n${$_(
            "requirements_button_bypass_warning_2",
          )}`,
          { title: "OpenGOAL Launcher", kind: "warning" },
        );
        if (confirmed) {
          await setBypassRequirements(true);
          dispatch("recheckRequirements");
        }
      }}>{$_("requirements_button_bypass")}</Button
    >
  </div>
</div>
