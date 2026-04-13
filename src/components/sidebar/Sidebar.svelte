<script lang="ts">
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  import logoJak3 from "$assets/images/jak-3.webp";
  import IconCog from "~icons/mdi/cog";
  import { Tooltip } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { jobTracker } from "$lib/stores/JobStore";
  import { route, isActive, navigate } from "../../router";

  let disabled = $derived($jobTracker?.overallStatus === "pending");

  function getNavItemStyle(itemName: any): string {
    const active = isActive.startsWith(itemName);

    return [
      "relative flex h-12 w-12 items-center justify-center rounded-lg",
      "transition-all duration-200 select-none",
      active
        ? "opacity-100"
        : "opacity-45 grayscale hover:opacity-80 hover:grayscale-0",
    ].join(" ");
  }
</script>

<div
  class="z-10 flex w-[72px] shrink-0 flex-col border border-white/5 bg-[#0b0b0b]"
>
  <ul class="flex h-full flex-col items-center gap-10 px-2 py-4">
    <li class="relative flex w-full justify-center">
      {#if route.pathname.startsWith("/jak1")}
        <div
          class="absolute left-[-8px] top-1/2 h-8 w-[3px] -translate-y-1/2 rounded-r bg-orange-500"
        ></div>
      {/if}
      <button
        id="jak1"
        class={getNavItemStyle("/jak1")}
        onclick={async () => {
          navigate(`/:game_name/`, {
            params: { game_name: "jak1" },
          });
        }}
        {disabled}
      >
        <img
          src={logoJak1}
          alt="Jak - The Precursor Legacy"
          aria-label="Jak - The Precursor Legacy"
        />
      </button>
      <Tooltip triggeredBy="#jak1" placement="right" type="dark">
        {$_("gameName_jak1")}
      </Tooltip>
    </li>

    <li class="relative flex w-full justify-center">
      {#if route.pathname.startsWith("/jak2")}
        <div
          class="absolute left-[-8px] top-1/2 h-8 w-[3px] -translate-y-1/2 rounded-r bg-orange-500"
        ></div>
      {/if}
      <button
        id="jak2"
        class={getNavItemStyle("/jak2")}
        onclick={async () => {
          navigate(`/:game_name/`, {
            params: { game_name: "jak2" },
          });
        }}
        {disabled}
      >
        <img src={logoJak2} alt="Jak 2" aria-label="Jak 2" />
      </button>
      <Tooltip triggeredBy="#jak2" placement="right" type="dark">
        {$_("gameName_jak2")}
      </Tooltip>
    </li>

    <li class="relative flex w-full justify-center">
      {#if route.pathname.startsWith("/jak3")}
        <div
          class="absolute left-[-8px] top-1/2 h-8 w-[3px] -translate-y-1/2 rounded-r bg-orange-500"
        ></div>
      {/if}
      <button
        id="jak3"
        class={getNavItemStyle("/jak3")}
        onclick={async () => {
          navigate(`/:game_name/`, {
            params: { game_name: "jak3" },
          });
        }}
        {disabled}
      >
        <img src={logoJak3} alt="Jak 3" aria-label="Jak 3" />
      </button>
      <Tooltip triggeredBy="#jak3" placement="right" type="dark">
        {$_("gameName_jak3")}
      </Tooltip>
    </li>

    <li class="mt-auto w-full border-t border-white/5 pt-4">
      <div class="relative flex justify-center">
        {#if route.pathname.startsWith("/settings")}
          <div
            class="absolute left-[-8px] top-1/2 h-8 w-[3px] -translate-y-1/2 rounded-r bg-orange-500"
          ></div>
        {/if}
        <button
          id="settings"
          class={getNavItemStyle("/settings")}
          onclick={() => {
            navigate(`/settings/:tab`, {
              params: { tab: "general" },
            });
          }}
          {disabled}
        >
          <IconCog class="text-[30px]" />
        </button>
      </div>
      <Tooltip triggeredBy="#settings" placement="right" type="dark">
        {$_("sidebar_settings")}
      </Tooltip>
    </li>
  </ul>
</div>
