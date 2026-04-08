<script lang="ts">
  import { Alert } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  let { activeGame } = $props();
</script>

<Alert
  rounded={false}
  class="bg-white dark:bg-slate-900 border-t-4 text-red-400"
>
  {#if activeGame === "jak2"}
    <span class="font-bold">{$_("gameControls_beta_headerA")}</span>
  {:else}
    <span class="font-bold">{$_("gameControls_beta_headerA_jak3")}</span>
  {/if}
  <em>{$_("gameControls_beta_headerB")}</em>
  <ul>
    <li>
      {$_("gameControls_beta_issueTracker_linkPreText")}
      <a
        class="text-blue-600"
        href="https://github.com/open-goal/jak-project/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3A{activeGame}"
        target="_blank"
        rel="noopener noreferrer"
        >{$_("gameControls_beta_issueTracker_linkText")}</a
      >
    </li>
    <li>
      {$_("gameControls_beta_bugReport_linkPreText")}
      <a
        class="text-blue-600"
        href="https://github.com/open-goal/jak-project/issues/new?template={activeGame}-bug-report.yml"
        target="_blank"
        rel="noopener noreferrer"
        >{$_("gameControls_beta_bugReport_linkText")}</a
      >
    </li>
  </ul>
</Alert>
