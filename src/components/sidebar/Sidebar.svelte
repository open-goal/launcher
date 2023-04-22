<script lang="ts">
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  import Icon from "@iconify/svelte";
  import { link, useLocation } from "svelte-navigator";
  import { Tooltip } from "flowbite-svelte";

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
    if (itemName === "jak1" && (pathName === "/jak1" || pathName === "/")) {
      return style;
    } else if (itemName === "jak2" && pathName === "/jak2") {
      return style;
    } else if (itemName === "jak3" && pathName === "/jak3") {
      return style;
    } else if (itemName === "jakx" && pathName === "/jakx") {
      return style;
    } else if (itemName === "settings" && pathName.startsWith("/settings")) {
      return style;
    } else if (itemName === "faq" && pathName === "/faq") {
      return style;
    }
    return style + " grayscale";
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
        <img src={logoJak1} alt="Jak - The Precursor Legacy" />
        <Tooltip placement="right"
          >Jak&nbsp;and&nbsp;Daxter:&nbsp;The&nbsp;Precursor&nbsp;Legacy</Tooltip
        >
      </a>
    </li>
    <li>
      <a
        class={getNavItemStyle("jak2", $location.pathname)}
        href="/jak2"
        use:link
      >
        <img src={logoJak2} alt="Jak 2" />
        <Tooltip placement="right" style="dark">Jak 2</Tooltip>
      </a>
    </li>
    <li class="fixed bottom-24 left-6">
      <a
        class={getNavItemStyle("settings", $location.pathname)}
        href="/settings/general"
        use:link
      >
        <Icon icon="material-symbols:settings" width={36} height={36} />
        <Tooltip placement="right" style="dark">Settings</Tooltip>
      </a>
    </li>

    <li class="fixed bottom-5 left-6">
      <a
        class={getNavItemStyle("faq", $location.pathname)}
        href="/faq"
        use:link
      >
        <Icon icon="material-symbols:contact-support" width={36} height={36} />
        <Tooltip placement="right" style="dark">Support&nbsp;&&nbsp;FAQ</Tooltip
        >
      </a>
    </li>
  </ul>
</div>
