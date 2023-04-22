<script lang="ts">
  import {
    getInstallationDirectory,
    resetLauncherSettingsToDefaults,
  } from "$lib/rpc/config";
  import { getActiveVersion, getActiveVersionFolder } from "$lib/rpc/versions";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { Button } from "flowbite-svelte";
  import { onMount } from "svelte";

  let currentInstallationDirectory = "";

  onMount(async () => {
    currentInstallationDirectory = await getInstallationDirectory();
  });
</script>

<div class="flex flex-col gap-2 mt-2">
  <div>
    <Button
      btnClass="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
      on:click={async () => {
        const confirmed = await confirm(
          "Are you sure you want to reset your launcher settings? This will reset your installation directory and any other settings you have changed. This will not affect your game files."
        );
        if (confirmed) {
          const result = resetLauncherSettingsToDefaults();
          if (result) {
            // TODO - move these to a store method
            $VersionStore.activeVersionType = await getActiveVersionFolder();
            $VersionStore.activeVersionName = await getActiveVersion();
          }
        }
      }}>Reset Launcher Settings</Button
    >
  </div>
</div>
