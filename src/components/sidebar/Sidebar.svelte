<script lang="ts">
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  import logoJak3 from "$assets/images/jak-3.webp";
  import IconCog from "~icons/mdi/cog";
  import IconChatQuestion from "~icons/mdi/chat-question";
  import { Tooltip } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { jobTracker } from "$lib/stores/JobStore";
  import { navigate, route } from "../../router";

  $: disabled = $jobTracker.overallStatus == "pending";

  function getNavStyle(pathName: string): string {
    const baseStyle = "grow-0 shrink-0 size-20 h-full bg-[#101010] px-1 z-10";
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

  function getNavItemStyle(itemName: string, pathName: string): string {
    const baseStyle =
      "hover:grayscale-0 hover:opacity-100 duration-500 text-orange-400 select-none";
    const isActive =
      pathName.startsWith(`/${itemName}`) ||
      (itemName === "jak1" && pathName === "/");
    return isActive ? baseStyle : `${baseStyle} grayscale`;
  }
</script>

<div class={getNavStyle(route.pathname)}>
  <ul class="flex flex-col h-full space-y-10 px-1 py-5 items-center">
    <li>
      <button
        id="jak1"
        class={getNavItemStyle("jak1", route.pathname)}
        onclick={async () => {
          navigate(`/:game_name`, { params: { game_name: "jak1" } });
        }}
        {disabled}
      >
        <img
          src={logoJak1}
          alt="Jak - The Precursor Legacy"
          aria-label="Jak - The Precursor Legacy"
        />
      </button>
      <Tooltip triggeredBy="#jak1" placement="right" type="dark"
        >{$_("gameName_jak1")}</Tooltip
      >
    </li>
    <li>
      <button
        id="jak2"
        class={getNavItemStyle("jak2", route.pathname)}
        onclick={async () => {
          navigate(`/:game_name`, { params: { game_name: "jak2" } });
        }}
        {disabled}
      >
        <img src={logoJak2} alt="Jak 2" aria-label="Jak 2" />
      </button>
      <Tooltip triggeredBy="#jak2" placement="right" type="dark"
        >{$_("gameName_jak2")}</Tooltip
      >
    </li>
    <li>
      <button
        id="jak3"
        class={getNavItemStyle("jak3", route.pathname)}
        onclick={async () => {
          navigate(`/:game_name`, { params: { game_name: "jak3" } });
        }}
        {disabled}
      >
        <img src={logoJak3} alt="Jak 3" aria-label="Jak 3" />
      </button>
      <Tooltip triggeredBy="#jak3" placement="right" type="dark"
        >{$_("gameName_jak3")}</Tooltip
      >
    </li>
    <li class="!mt-auto">
      <button
        id="settings"
        class={getNavItemStyle("settings", route.pathname)}
        onclick={async () => {
          navigate(`/settings/:tab`, { params: { tab: "general" } });
        }}
        {disabled}
      >
        <IconCog style="font-size: 36px" />
      </button>
      <Tooltip triggeredBy="#settings" placement="right" type="dark"
        >{$_("sidebar_settings")}</Tooltip
      >
    </li>

    <li>
      <button
        id="faq"
        class={getNavItemStyle("faq", route.pathname)}
        onclick={async () => {
          navigate(`/faq`);
        }}
        {disabled}
      >
        <IconChatQuestion style="font-size: 36px" />
      </button>
      <Tooltip triggeredBy="#faq" placement="right" type="dark"
        >{$_("sidebar_help")}</Tooltip
      >
    </li>
  </ul>
</div>
