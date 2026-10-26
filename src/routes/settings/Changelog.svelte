<script lang="ts">
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import {
    listLauncherReleases,
    type LauncherRelease,
  } from "$lib/utils/github";
  import ChangelogTable from "../../components/changelog/ChangelogTable.svelte";
  import { _ } from "svelte-i18n";
  import { Alert, Badge, Button, Spinner, Toggle } from "flowbite-svelte";
  import IconCalendar from "~icons/mdi/CalendarMonth";
  import IconCheck from "~icons/mdi/CheckBold";
  import IconGitHub from "~icons/mdi/github";
  import IconRefresh from "~icons/mdi/refresh";
  import { config } from "/src/state/config.svelte";

  let locale = $state(config?.locale || "en-US");
  let loading = $state(true);
  let releases: LauncherRelease[] = $state([]);
  let launcherVersion: string | undefined = $state(undefined);
  let showDependencyChanges = $state(false);

  onMount(async () => {
    launcherVersion = `v${await getVersion()}`;
    await refreshReleases();
  });

  async function refreshReleases() {
    loading = true;
    releases = await listLauncherReleases();
    loading = false;
  }

  // hide releases that would otherwise show an empty table
  const visibleReleases = $derived(
    releases.filter((release) =>
      release.changes.some(
        (note) =>
          showDependencyChanges || !note.contributor.includes("dependabot"),
      ),
    ),
  );
</script>

<p class="text-sm mt-2 mb-2">{$_("settings_changelog_header")}</p>
{#if loading}
  <div class="flex flex-col justify-center items-center">
    <Spinner color="yellow" size="12" />
  </div>
{:else if releases.length > 0}
  <div class="flex items-center mb-2">
    <div class="grow">
      <Toggle
        checked={showDependencyChanges}
        color="orange"
        onchange={(evt: any) => {
          showDependencyChanges = evt.target.checked;
        }}>{$_("update_button_hideDependencyChanges")}</Toggle
      >
    </div>
    <Button
      class="p-2! rounded-md bg-orange-500 hover:bg-orange-600 text-slate-900"
      onclick={refreshReleases}
    >
      <IconRefresh aria-label={$_("settings_changelog_icon_refresh_altText")} />
    </Button>
  </div>

  {#each visibleReleases as release (release.version)}
    <div class="mt-6">
      <div class="flex items-center gap-4">
        <h2 class="text-xl font-bold tracking-wide text-gray-200">
          {release.version}
        </h2>
        {#if release.version === launcherVersion}
          <Badge color="green" class="w-fit gap-1 tracking-wide">
            <IconCheck />
            {$_("settings_changelog_currentVersion")}
          </Badge>
        {/if}
        {#if release.date}
          <p
            class="flex items-center gap-1 tracking-wide text-gray-300 text-sm"
          >
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
  {/each}
{:else}
  <Alert class="bg-slate-900 grow text-red-400">
    {$_("settings_changelog_noReleasesFound")}
  </Alert>
{/if}
