<script lang="ts">
  import { page } from "$app/state";
  import { Tooltip } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import IconCog from "~icons/mdi/cog";
  import IconChatQuestion from "~icons/mdi/chat-question";

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
    return baseStyle; // early return so i can develop more important things
    const isActive =
      pathName.startsWith(`${itemName}`) ||
      (itemName === "jak1" && pathName === "/");
    return isActive ? baseStyle : `${baseStyle} grayscale`;
  }
</script>

<aside class={getNavStyle("$location.pathname")}>
  <ul class="flex flex-col justify-between h-full px-1 items-center text-black">
    <div class="flex flex-col space-y-16 pt-4">
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
          >$_{$_("gameName_jak1")}</Tooltip
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
          >{$_("gameName_jak2")}</Tooltip
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
          >{$_("gameName_jak3")}</Tooltip
        >
      </li>
    </div>

    <div class="flex flex-col items-center space-y-16 pb-16">
      <li>
        <a
          id="settings"
          class={getNavItemStyle("settings", "$location.pathname")}
          href="/settings/general"
        >
          <IconCog style="font-size: 36px" />
        </a>
        <Tooltip triggeredBy="#settings" placement="right" type="dark"
          >{$_("sidebar_settings")}</Tooltip
        >
      </li>

      <li class="pb-2">
        <a
          id="faq"
          class={getNavItemStyle("faq", "$location.pathname")}
          href="/faq"
        >
          <IconChatQuestion style="font-size: 36px" />
        </a>
        <Tooltip triggeredBy="#faq" placement="right" type="dark"
          >{$_("sidebar_help")}</Tooltip
        >
      </li>
    </div>
  </ul>
</aside>
