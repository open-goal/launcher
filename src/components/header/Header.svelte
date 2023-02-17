<script>
  import { appWindow } from "@tauri-apps/api/window";
  import icon from "$assets/images/icon.png";
  import { onMount } from "svelte";
  import { launcherConfig } from "$lib/config";
  import { SupportedGame } from "$lib/constants";
  import { getVersion } from "@tauri-apps/api/app";
  import { Link } from "svelte-navigator";
  import { Tooltip } from "flowbite-svelte";
  import Icon from "@iconify/svelte";
  import { handleCheckUpdate } from "$lib/utils/updates";
  import { UpdateStore } from "$lib/stores/AppStore";
  let gameVersion;
  let launcherVerison;
  // TODO: get the active game rigged up here properly
  let activeGame = SupportedGame.Jak1;

  onMount(async () => {
    // TODO - check for updates
    gameVersion = await launcherConfig.getGameInstallVersion(activeGame);
    if (gameVersion.startsWith("v")) {
      gameVersion = gameVersion.slice(1);
    }
    launcherVerison = await getVersion();
    await handleCheckUpdate();
  });
</script>

<header
  class="flex flex-row bg-[#101010] pl-2 pr-4 pt-1 pb-1 items-center"
  data-tauri-drag-region
>
  <div class="flex flex-row items-center space-x-2 pointer-events-none">
    <img class="h-8" src={icon} alt="" />
    <p class="font-black tracking-tight text-lg">OpenGOAL</p>
  </div>

  <div class="border-l border-[#9f9f9f] h-8 m-2" />

  <div class="flex flex-col text-neutral-300 mr-2 pointer-events-none">
    <p class="font-mono text-sm">v{launcherVerison}</p>
    <p class="font-mono text-sm">v{gameVersion}</p>
  </div>

  <div class="flex flex-col text-neutral-500 mr-2 pointer-events-none">
    <p class="font-mono text-sm">Launcher</p>
    <p class="font-mono text-sm">Tooling</p>
  </div>

  <div class="flex flex-col text-orange-500">
    <p class="font-mono text-sm hover:text-orange-300">
      <!-- TODO - gotta make a tauri update page -->
      <Link class="font-mono" to="TODO">> Update Available!</Link>
    </p>
    <p class="font-mono text-sm hover:text-orange-300">
      <Link class="font-mono" to="/settings/versions">> Update Available!</Link>
    </p>
  </div>

  <div class="flex space-x-4 text-xl ml-auto">
    <button class="hover:text-amber-600" on:click={() => appWindow.minimize()}>
      <Icon icon="material-symbols:chrome-minimize" width="24" height="24" />
    </button>
    <button class="hover:text-red-600" on:click={() => appWindow.close()}>
      <Icon icon="ic:baseline-close" width="24" height="24" />
    </button>
  </div>
</header>
