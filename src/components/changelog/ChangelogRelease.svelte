<script lang="ts">
  import { isDependencyChange, type LauncherRelease } from "$lib/utils/github";
  import ChangelogTable from "./ChangelogTable.svelte";
  import { _ } from "svelte-i18n";
  import IconCalendar from "~icons/mdi/CalendarMonth";
  import IconGitHub from "~icons/mdi/github";
  import { config } from "/src/state/config.svelte";

  let {
    release,
    showDependencyChanges,
  }: { release: LauncherRelease; showDependencyChanges: boolean } = $props();

  let locale = $state(config?.locale || "en-US");
</script>

<!-- skip releases that would otherwise show an empty table -->
{#if release.changes.some((note) => showDependencyChanges || !isDependencyChange(note))}
  <div class="mt-6">
    <div class="flex items-center gap-4">
      <h2 class="text-xl font-bold tracking-wide text-gray-200">
        {release.version}
      </h2>
      {#if release.date}
        <p class="flex items-center gap-1 tracking-wide text-gray-300 text-sm">
          <IconCalendar />
          {new Date(release.date).toLocaleString(locale, {
            month: "short",
            day: "numeric",
            year: "numeric",
          })}
        </p>
      {/if}
      <a
        href={release.githubLink}
        target="_blank"
        rel="noreferrer"
        class="flex items-center gap-1 cursor-pointer tracking-wide text-gray-300 hover:text-white text-sm capitalize"
      >
        <IconGitHub />
        {$_("settings_versions_icon_githubRelease_altText")}
      </a>
    </div>
    <ChangelogTable changes={release.changes} {showDependencyChanges} />
  </div>
{/if}
