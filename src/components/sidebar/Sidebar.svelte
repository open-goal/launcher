<script lang="ts">
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  import logoJak3 from "$assets/images/jak-3.webp";
  import IconCog from "~icons/mdi/cog";
  import IconChatQuestion from "~icons/mdi/chat-question";
  import { link, useLocation } from "svelte-navigator";
  import { Tooltip } from "flowbite-svelte";
  import { _ } from "svelte-i18n";

  const location = useLocation();
  $: $location.pathname;

  function getNavStyle(pathname: string): string {
    const baseStyle =
      "grow-0 shrink-0 basis-1/10 h-full bg-[#101010] px-1 z-10";
    const isOpaque =
      pathname.startsWith("/settings") ||
      pathname.startsWith("/faq") ||
      pathname.startsWith("/update");
    return isOpaque
      ? baseStyle
      : `${baseStyle} opacity-50 hover:opacity-100 duration-500`;
  }

  function getNavItemStyle(itemName: string, pathName: string): string {
    const baseStyle =
      "flex items-center hover:grayscale-0 hover:opacity-100 duration-500 text-orange-400";
    const isActive =
      pathName.startsWith(`/${itemName}`) ||
      (itemName === "jak1" && pathName === "/");
    return isActive ? baseStyle : `${baseStyle} grayscale`;
  }
</script>

<div class={getNavStyle($location.pathname)}>
  <ul class="flex flex-col h-full space-y-10 px-1 py-5 items-center">
    <li>
      <a
        id="jak1"
        class={getNavItemStyle("jak1", $location.pathname)}
        href="/jak1"
        use:link
      >
        <img
          src={logoJak1}
          alt="Jak - The Precursor Legacy"
          aria-label="Jak - The Precursor Legacy"
        />
      </a>
      <Tooltip triggeredBy="#jak1" placement="right" type="dark"
        >{$_("gameName_jak1")}</Tooltip
      >
    </li>
    <li>
      <a
        id="jak2"
        class={getNavItemStyle("jak2", $location.pathname)}
        href="/jak2"
        use:link
      >
        <img src={logoJak2} alt="Jak 2" aria-label="Jak 2" />
      </a>
      <Tooltip triggeredBy="#jak2" placement="right" type="dark"
        >{$_("gameName_jak2")}</Tooltip
      >
    </li>
    <li>
      <a
        id="jak3"
        class={getNavItemStyle("jak3", $location.pathname)}
        href="/jak3"
        use:link
      >
        <img src={logoJak3} alt="Jak 3" aria-label="Jak 3" />
      </a>
      <Tooltip triggeredBy="#jak3" placement="right" type="dark"
        >{$_("gameName_jak3")}</Tooltip
      >
    </li>
    <li class="!mt-auto">
      <a
        id="settings"
        class={getNavItemStyle("settings", $location.pathname)}
        href="/settings/general"
        use:link
      >
        <IconCog style="font-size: 36px" />
      </a>
      <Tooltip triggeredBy="#settings" placement="right" type="dark"
        >{$_("sidebar_settings")}</Tooltip
      >
    </li>

    <li>
      <a
        id="faq"
        class={getNavItemStyle("faq", $location.pathname)}
        href="/faq"
        use:link
      >
        <IconChatQuestion style="font-size: 36px" />
      </a>
      <Tooltip triggeredBy="#faq" placement="right" type="dark"
        >{$_("sidebar_help")}</Tooltip
      >
    </li>
  </ul>
</div>
