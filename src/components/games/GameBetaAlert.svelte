<script lang="ts">
  import { Alert } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
</script>

<Alert rounded={false} class="border-t-4 text-red-400">
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
</Alert>
