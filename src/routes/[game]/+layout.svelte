<script lang="ts">
  import type { LayoutProps } from "./$types";
  import { page } from "$app/state";
  import GameInProgress from "../../components/GameInProgress.svelte";

  let { data, children }: LayoutProps = $props();
  const game = $derived(page.params.game);
  $effect(() => localStorage.setItem("lastGame", game));
</script>

<!-- BACKGROUND -->
<img
  class="absolute right-0 top-0 w-screen h-screen -z-100"
  src={`/images/${game}/background.webp`}
  alt=""
/>

<!-- TODO: get rid of this conditional when jak3 is released, keep {@render children()} -->
{#if game == "jak3"}
  <GameInProgress></GameInProgress>
{:else}
  {@render children()}
{/if}

<!-- Jak 2 BETA warning -->
<!-- <Alert rounded={false} class="border-t-4 text-red-400">
  <span class="font-bold">{$_("gameControls_beta_headerA")}</span>
  <em>{$_("gameControls_beta_headerB")}</em>
  <br />
  <ul>
    <li>
      {$_("gameControls_beta_issueTracker_linkPreText")}
      <a
        class="text-blue-400"
        href="https://github.com/open-goal/jak-project/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3Ajak2"
        target="_blank"
        rel="noopener noreferrer"
        >{$_("gameControls_beta_issueTracker_linkText")}</a
      >
    </li>
    <li>
      {$_("gameControls_beta_bugReport_linkPreText")}
      <a
        class="text-blue-400"
        href="https://github.com/open-goal/jak-project/issues/new?template=jak2-bug-report.yml"
        target="_blank"
        rel="noopener noreferrer"
        >{$_("gameControls_beta_bugReport_linkText")}</a
      >
    </li>
  </ul>
</Alert> -->

<!-- Jak 3 BETA warning -->
<!-- <Alert rounded={false} class="border-t-4 text-red-400">
  <span class="font-bold">{$_("gameControls_beta_headerA_jak3")}</span>
  <em>{$_("gameControls_beta_headerB")}</em>
  <br />
  <ul>
    <li>
      {$_("gameControls_beta_issueTracker_linkPreText")}
      <a
        class="text-blue-400"
        href="https://github.com/open-goal/jak-project/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3Ajak3"
        target="_blank"
        rel="noopener noreferrer"
        >{$_("gameControls_beta_issueTracker_linkText")}</a
      >
    </li>
    <li>
      {$_("gameControls_beta_bugReport_linkPreText")}
      <a
        class="text-blue-400"
        href="https://github.com/open-goal/jak-project/issues/new?template=jak3-bug-report.yml"
        target="_blank"
        rel="noopener noreferrer"
        >{$_("gameControls_beta_bugReport_linkText")}</a
      >
    </li>
  </ul>
</Alert> -->
