<script>
  import { appWindow } from "@tauri-apps/api/window";
  import icon from "$assets/images/icon.png";
  import { onMount } from "svelte";
  import { launcherConfig } from "$lib/config";
  import { SupportedGame } from "$lib/constants";
  import { getVersion } from "@tauri-apps/api/app";
  let gameVersion;
  let launcherVerison;
  // TODO: get the active game rigged up here properly
  let activeGame = SupportedGame.Jak1;

  onMount(async () => {
    gameVersion = await launcherConfig.getGameInstallVersion(activeGame);
    launcherVerison = await getVersion();
  });
</script>

<header
  class="flex flex-row h-12 bg-[#101010] pl-2 pr-4 items-center"
  data-tauri-drag-region
>
  <div class="flex flex-row items-center space-x-2 text-2xl">
    <img class="h-10" src={icon} alt="" />
    <p>OpenGOAL</p>
  </div>

  <div class="border-l border-[#9f9f9f] h-10 m-2" />

  <div class="flex flex-col space-y-0 text-[#9f9f9f]">
    <p>v{launcherVerison} Launcher</p>
    <p>{gameVersion} Tools</p>
  </div>

  <div class="flex space-x-4 text-xl ml-auto">
    <!-- TODO: Conditional rendering of the alert bell. One case would be if there is an update available. -->
    <i class="fa-solid fa-bell hover:text-emerald-600" />
    <i
      class="fa fa-window-minimize hover:text-amber-600"
      on:click={() => appWindow.minimize()}
    />
    <i
      class="fa-solid fa-times hover:text-red-600"
      on:click={() => appWindow.close()}
    />
  </div>
</header>
