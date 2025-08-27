<script lang="ts">
  import type { LayoutProps } from "./$types";
  import "../app.css";
  import { Tooltip } from "flowbite-svelte";
  import IconCog from "~icons/mdi/cog";
  import IconChatQuestion from "~icons/mdi/chat-question";
  import IconWindowMinimize from "~icons/mdi/window-minimize";
  import IconWindowClose from "~icons/mdi/window-close";
  import { page } from "$app/state";
  import { check } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { ask } from "@tauri-apps/plugin-dialog";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  let { data, children }: LayoutProps = $props();
  const activeGame = $derived(page.params.game);

  function getNavStyle(pathName: string): string {
    const baseStyle = "w-20 h-screen bg-[#101010] z-100";
    const isOpaque =
      pathName.startsWith("/settings") ||
      pathName.startsWith("/update") ||
      pathName.startsWith("/faq") ||
      pathName.endsWith("/mods") ||
      pathName.endsWith("/texture_packs");
    return isOpaque
      ? baseStyle
      : `${baseStyle} opacity-50 hover:opacity-100 duration-500`;
  }

  // TODO: Fix this function
  function getNavItemStyle(itemName: string, pathName: string): string {
    const baseStyle =
      "hover:grayscale-0 hover:opacity-100 duration-500 text-orange-400";
    const isActive =
      pathName.startsWith(`${itemName}`) ||
      (itemName === "jak1" && pathName === "/");
    return isActive ? baseStyle : `${baseStyle} grayscale`;
  }

  let launcherVerison = 420;
  const appWindow = getCurrentWebviewWindow();
</script>

<div class="flex h-screen flex-col">
  <!-- HEADER -->
  <header
    class="flex flex-row grow-0 shrink-0 bg-[#101010] pl-2 pr-4 pt-1 pb-1 items-center z-10"
    data-tauri-drag-region
  >
    <div
      class="flex flex-row shrink-0 items-center space-x-2 pointer-events-none"
    >
      <img
        class="h-8"
        src="/images/icon.webp"
        alt="OpenGOAL logo"
        aria-label="OpenGOAL logo"
      />
      <p class="font-black text-white tracking-tight text-lg">OpenGOAL</p>
    </div>
    <div class="border-l shrink-0 border-[#9f9f9f] h-8 m-2"></div>
    <div
      class="flex flex-col shrink-0 text-neutral-500 mr-2 pointer-events-none"
    >
      <p class="font-mono text-sm">{"header_launcherVersionLabel"}</p>
      <p class="font-mono text-sm">{"header_toolingVersionLabel"}</p>
    </div>
    <div
      class="flex flex-col text-neutral-300 mr-2 pointer-events-none max-w-[250px]"
    >
      <p class="font-mono text-sm">
        {launcherVerison}
      </p>
      <!-- <p class="font-mono text-sm">
        {$VersionStore.activeVersionName === null
          ? "not set!"
          : $VersionStore.activeVersionName}
      </p> -->
    </div>
    <!-- {#if $UpdateStore.selectedTooling.updateAvailable}
      <a
        class="font-mono text-sm mt-5 text-orange-500 hover:text-orange-300"
        href="/settings/versions">{("header_updateAvailable")}</a
      >
    {/if} -->
    <div class="flex shrink-0 space-x-4 text-xl ml-auto">
      <button
        class="text-white hover:text-amber-600"
        onclick={() => appWindow.minimize()}
      >
        <IconWindowMinimize />
      </button>
      <button
        class="text-white hover:text-red-600"
        onclick={() => appWindow.close()}
      >
        <IconWindowClose />
      </button>
    </div>
  </header>

  <div class="flex">
    <!-- SIDEBAR  -->
    <aside class={getNavStyle("$location.pathname")}>
      <ul class="flex flex-col h-full space-y-10 px-1 py-5 items-center">
        <li>
          <a
            id="jak1"
            class={getNavItemStyle("jak1", activeGame)}
            href="/game/jak1"
          >
            <img
              src="/images/jak-tpl.webp"
              alt="Jak - The Precursor Legacy"
              aria-label="Jak - The Precursor Legacy"
            />
          </a>
          <Tooltip triggeredBy="#jak1" placement="right" type="dark"
            >{"gameName_jak1"}</Tooltip
          >
        </li>
        <li>
          <a
            id="jak2"
            class={getNavItemStyle("jak2", activeGame)}
            href="/game/jak2"
          >
            <img src="/images/jak-2.webp" alt="Jak 2" aria-label="Jak 2" />
          </a>
          <Tooltip triggeredBy="#jak2" placement="right" type="dark"
            >{"gameName_jak2"}</Tooltip
          >
        </li>
        <li>
          <a
            id="jak3"
            class={getNavItemStyle("jak3", activeGame)}
            href="/game/jak3"
          >
            <img src="/images/jak-3.webp" alt="Jak 3" aria-label="Jak 3" />
          </a>
          <Tooltip triggeredBy="#jak3" placement="right" type="dark"
            >{"gameName_jak3"}</Tooltip
          >
        </li>
        <li class="!mt-auto">
          <a
            id="settings"
            class={getNavItemStyle("settings", "$location.pathname")}
            href="/settings/general"
          >
            <IconCog style="font-size: 36px" />
          </a>
          <Tooltip triggeredBy="#settings" placement="right" type="dark"
            >{"sidebar_settings"}</Tooltip
          >
        </li>

        <li>
          <a
            id="faq"
            class={getNavItemStyle("faq", "$location.pathname")}
            href="/faq"
          >
            <IconChatQuestion style="font-size: 36px" />
          </a>
          <Tooltip triggeredBy="#faq" placement="right" type="dark"
            >{"sidebar_help"}</Tooltip
          >
        </li>
      </ul>
    </aside>

    <!-- BODY -->
    <main class="flex-1 overflow-hidden">
      {@render children()}
    </main>
  </div>
</div>
