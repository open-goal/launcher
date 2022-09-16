<script>
  import { appWindow } from "@tauri-apps/api/window";
  import icon from "$assets/images/icon.png";
  import { onMount } from "svelte";
  import { launcherConfig } from "$lib/config";
  import { SupportedGame } from "$lib/constants";
  import { getVersion } from "@tauri-apps/api/app";
  import { link } from "svelte-navigator";
  import { Tooltip } from "flowbite-svelte";
  import { handleCheckUpdate } from "$lib/utils/updates";
  import { UpdateStore } from "$lib/stores/AppStore";
  let gameVersion;
  let launcherVerison;
  // TODO: get the active game rigged up here properly
  let activeGame = SupportedGame.Jak1;

  onMount(async () => {
    gameVersion = await launcherConfig.getGameInstallVersion(activeGame);
    launcherVerison = await getVersion();
    await handleCheckUpdate();
  });
</script>

<header
  class="flex flex-row h-10 bg-[#101010] pl-2 pr-4 items-center"
  data-tauri-drag-region
>
  <div class="flex flex-row items-center space-x-2 text-2xl">
    <img class="h-8" src={icon} alt="" />
    <p>OpenGOAL</p>
  </div>

  <div class="border-l border-[#9f9f9f] h-8 m-2" />

  <div class="flex flex-col text-[#9f9f9f]">
    <p class="text-sm">v{launcherVerison} Launcher</p>
    <p class="text-sm">{gameVersion} Tools</p>
  </div>

  <div class="flex space-x-4 text-xl ml-auto">
    {#if $UpdateStore.shouldUpdate}
      <a href="/update" use:link
        ><i
          class="fa-solid fa-bell text-yellow-300 animate-pulse hover:cursor-pointer hover:animate-none"
        />
        <Tooltip arrow={false} placement="bottom" style="dark"
          >Update Available</Tooltip
        >
      </a>
    {/if}
    <i
      class="fa fa-window-minimize hover:text-amber-600 hover:cursor-pointer"
      on:click={() => appWindow.minimize()}
    />
    <i
      class="fa-solid fa-times hover:text-red-600 hover:cursor-pointer"
      on:click={() => appWindow.close()}
    />
  </div>
</header>
