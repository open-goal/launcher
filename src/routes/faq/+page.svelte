<script lang="ts">
  import type { PageProps } from "./$types";
  import { Button, Spinner } from "flowbite-svelte";
  import IconDiscord from "~icons/mdi/discord";
  import IconGitHub from "~icons/mdi/github";
  import { generateSupportPackage } from "$lib/rpc/support";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import { onMount } from "svelte";
  import { appLogDir } from "@tauri-apps/api/path";
  import { _ } from "svelte-i18n";

  let appLogDirPath = $state("");
  let downloadingPackage = $state(false);

  let { data }: PageProps = $props();
</script>

<div class="flex flex-col h-full bg-slate-900 p-4 gap-3">
  <h1 class="font-semibold text-xl text-orange-500">{$_("help_header")}</h1>
  <p class="text-sm">
    {$_("help_foreword")}
  </p>
  <div class="flex flex-row mt-1 gap-2">
    <Button
      disabled={downloadingPackage}
      class="border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-4 py-2"
      onclick={async () => {
        downloadingPackage = true;
        await generateSupportPackage();
        downloadingPackage = false;
      }}
    >
      {#if downloadingPackage}
        <Spinner class="text-sm mb-0.5 mr-1" size="4" color="yellow" />
      {/if}
      {$_("help_button_downloadPackage")}</Button
    >
    {#if appLogDirPath}
      <Button
        class="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-4 py-2"
        onclick={() => {
          revealItemInDir(appLogDirPath);
        }}>{$_("help_button_openLogFolder")}</Button
      >
    {/if}
    <Button
      class="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-4 py-2"
      href="https://raw.githubusercontent.com/open-goal/launcher/refs/heads/main/docs/default-keybinds.png"
      target="_blank"
      rel="noreferrer noopener">{$_("help_button_defaultKeybinds")}</Button
    >
  </div>
  <p class="mt-3 text-sm">
    {$_("help_description_createAnIssue")}
  </p>
  <p class="text-sm">
    {$_("help_description_duplicateReminder")}
  </p>
  <div class="flex flex-row gap-2">
    <Button
      class="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-4 py-2"
      href="https://discord.gg/dPRCfsju3N"
      target="_blank"
      rel="noreferrer noopener"
    >
      <IconDiscord />&nbsp;Discord
    </Button>
    <Button
      class="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-4 py-2"
      href="https://github.com/open-goal/launcher/issues/new/choose"
      target="_blank"
      rel="noreferrer noopener"
    >
      <IconGitHub />&nbsp;{$_("help_button_reportLauncherIssue")}
    </Button>
    <Button
      class="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-4 py-2"
      href="https://github.com/open-goal/jak-project/issues/new/choose"
      target="_blank"
      rel="noreferrer noopener"
    >
      <IconGitHub />&nbsp;{$_("help_button_reportGameIssue")}
    </Button>
  </div>
  <div class="flex mt-auto justify-end">
    <p class="text-gray-500 text-xs text-center">
      All third party trademarks (including but not limited to: logos and icons)
      referenced by OpenGOAL remain the property of their respective owners.
      Unless specifically identified as such, OpenGOAL's use of third party
      trademarks does not indicate any relationship, sponsorship, or endorsment
      between OpenGOAL and the owners of these trademarks. Any references to the
      aforementioned trademarks are for informative purposes and should be
      considered nominative fair use.
    </p>
  </div>
</div>
