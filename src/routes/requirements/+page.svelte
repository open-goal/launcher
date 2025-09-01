<script lang="ts">
  import { Alert, Button } from "flowbite-svelte";
  import type { PageProps } from "./$types";
  import { _ } from "svelte-i18n";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import {
    isAVXRequirementMet,
    isOpenGLRequirementMet,
    setBypassRequirements,
  } from "$lib/rpc/config";
  import { goto, invalidateAll } from "$app/navigation";
  import { type } from "@tauri-apps/plugin-os";

  let { data }: PageProps = $props();
  const game = $derived(data.game);
  const avxMet = data.avx;
  const openGLMet = data.openGL;
  const diskSpaceMet = data.disk;
  const vccMet = data.vcc;
  const ARMOutsideMac = data.ARMOutsideMac;
  const isMinMacOSVersion = data.isMacOSMin;
</script>

<div class="flex flex-col h-full items-center p-4 text-center bg-zinc-900">
  <h1 class="text-xl font-black mb-5 text-outline">
    {$_("requirements_notMet_header")}
  </h1>

  <div class="flex flex-col gap-2 overflow-y-scroll">
    {#if !avxMet}
      <Alert class="text-start" rounded={false} color="red">
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
      </Alert>
    {/if}

    {#if !openGLMet}
      <Alert class="text-start" rounded={false} color="red">
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
      </Alert>
    {/if}

    {#if !diskSpaceMet}
      <Alert class="text-start" rounded={false} color="red">
        <span class="font-bold"
          >{$_(`requirements_disk_notEnoughSpace_${game}`)}</span
        >
      </Alert>
    {/if}

    {#if !vccMet}
      <Alert class="text-start" rounded={false} color="red">
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
      </Alert>
    {/if}

    {#if ARMOutsideMac}
      <Alert class="text-start" rounded={false} color="red">
        <span class="font-bold"
          >{$_("requirements_armNotSupportedOutsideMacOS")}</span
        >
      </Alert>
    {/if}

    {#if !isMinMacOSVersion && type() == "macos"}
      <Alert class="text-start" rounded={false} color="red">
        <span class="font-bold"
          >{$_("requirements_macos_notAtleastVersion15")}</span
        >
      </Alert>
    {/if}

    <div>
      <Button
        class="border-solid border-2 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          await isAVXRequirementMet();
          await isOpenGLRequirementMet(true);
          await invalidateAll();
        }}
        >{$_("requirements_button_recheck")}
      </Button>
      <Button
        class="border-solid border-2 rounded bg-orange-800 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          const confirmed = await confirm(
            `${$_("requirements_button_bypass_warning_1")}\n\n${$_(
              "requirements_button_bypass_warning_2",
            )}`,
            { title: "OpenGOAL Launcher", kind: "warning" },
          );
          if (confirmed) {
            await setBypassRequirements(true);
            await invalidateAll();
            goto(`${game}`);
          }
        }}
        >{$_("requirements_button_bypass")}
      </Button>
    </div>
  </div>
</div>
