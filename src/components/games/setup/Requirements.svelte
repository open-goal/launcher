<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import {
    isAVXRequirementMet,
    isOpenGLRequirementMet,
    setBypassRequirements,
  } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { confirm } from "@tauri-apps/api/dialog";

  let isAVXMet = false;
  let isOpenGLMet = false;

  const dispatch = createEventDispatcher();

  onMount(async () => {
    isAVXMet = await isAVXRequirementMet(false);
    isOpenGLMet = await isOpenGLRequirementMet(false);
  });

  function alertColor(val: boolean | undefined) {
    if (val === undefined) {
      return "yellow";
    }
    return val ? "green" : "red";
  }
</script>

<div
  class="flex flex-col h-full justify-center items-center p-5 text-center gap-3"
>
  <h1 class="text-xl font-black mb-5 text-outline">
    {$_("requirements_notMet_header")}
  </h1>
  <Alert
    class="w-full text-start"
    accent
    rounded={false}
    color={alertColor(isAVXMet)}
  >
    {#if isAVXMet}
      <span class="font-bold">{$_("requirements_cpu_supportsAVX")}</span>
    {:else if isAVXMet === undefined}
      <span class="font-bold">{$_("requirements_cpu_unableToCheckAVX")}</span>
    {:else}
      <span class="font-bold">{$_("requirements_cpu_doesNotSupportAVX")}</span>
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
  <Alert
    class="w-full text-start"
    accent
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
  <div>
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={async () => {
        isAVXMet = await isAVXRequirementMet(true);
        isOpenGLMet = await isOpenGLRequirementMet(true);
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
          { title: "OpenGOAL Launcher", type: "warning" },
        );
        if (confirmed) {
          await setBypassRequirements(true);
          dispatch("recheckRequirements");
        }
      }}>{$_("requirements_button_bypass")}</Button
    >
  </div>
</div>
