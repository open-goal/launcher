<script lang="ts">
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  import Icon from "@iconify/svelte";
  import { link, useLocation } from "svelte-navigator";
  import { Tooltip } from "flowbite-svelte";
  import { SupportedGame, getInternalName } from "$lib/constants";
  import { _ } from "svelte-i18n";

  const location = useLocation();
  $: $location.pathname;

  function getNavStyle(pathname: string): string {
    let style = "basis-1/10 h-full bg-[#101010] px-1 z-10";
    if (
      !pathname.startsWith("/settings") &&
      !pathname.startsWith("/faq") &&
      !pathname.startsWith("/update")
    ) {
      style += " opacity-50 hover:opacity-100 duration-500";
    }
    return style;
  }

  function getNavItemStyle(itemName: string, pathName: string): string {
    let style =
      "flex items-center hover:grayscale-0 hover:opacity-100 duration-500 text-orange-400 duration-500";
    if (
      itemName === "jak1" &&
      (pathName.startsWith("/jak1") || pathName === "/")
    ) {
      return style;
    } else if (itemName === "jak2" && pathName.startsWith("/jak2")) {
      return style;
    } else if (itemName === "jak3" && pathName.startsWith("/jak3")) {
      return style;
    } else if (itemName === "jakx" && pathName.startsWith("/jakx")) {
      return style;
    } else if (itemName === "settings" && pathName.startsWith("/settings")) {
      return style;
    } else if (itemName === "faq" && pathName === "/faq") {
      return style;
    }
    return style + " grayscale";
  }

  function modifyGameTitleName(gameName: string): string {
    // I figured out how to modify the tooltip size, the problem is that we want the tooltip to adjust based on the size
    // new solution:
    // - insert non-breaking-spaces to any space in the translated name
    // - don't insert a nbsp after a `:`
    return gameName.replace(/(?:[^:])\s/g, (match) =>
      match.replace(/\s/g, "\u00a0")
    );
  }
</script>

<div class={getNavStyle($location.pathname)}>
  <ul class="flex flex-col space-y-12 px-1 py-5">
    <li>
      <a
        class={getNavItemStyle("jak1", $location.pathname)}
        href="/jak1"
        use:link
      >
        <img src={logoJak1} aria-label="Jak - The Precursor Legacy" />
        <Tooltip
          placement="right"
          style="custom"
          class="text-center py-2 px-3 text-sm font-medium"
          >{modifyGameTitleName(
            $_(`gameName_${getInternalName(SupportedGame.Jak1)}`)
          )}</Tooltip
        >
      </a>
    </li>
    <li>
      <a
        class={getNavItemStyle("jak2", $location.pathname)}
        href="/jak2"
        use:link
      >
        <img src={logoJak2} aria-label="Jak 2" />
        <Tooltip placement="right" style="dark"
          >{$_(`gameName_${getInternalName(SupportedGame.Jak2)}`)}</Tooltip
        >
      </a>
    </li>
    <li class="fixed bottom-24 left-6">
      <a
        class={getNavItemStyle("settings", $location.pathname)}
        href="/settings/general"
        use:link
      >
        <Icon icon="material-symbols:settings" width={36} height={36} />
        <Tooltip placement="right" style="dark"
          >{$_("sidebar_settings")}</Tooltip
        >
      </a>
    </li>

    <li class="fixed bottom-5 left-6">
      <a
        class={getNavItemStyle("faq", $location.pathname)}
        href="/faq"
        use:link
      >
        <Icon icon="material-symbols:contact-support" width={36} height={36} />
        <Tooltip placement="right" style="dark">{$_("sidebar_help")}</Tooltip>
      </a>
    </li>
  </ul>
</div>
