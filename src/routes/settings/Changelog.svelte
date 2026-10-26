<script lang="ts">
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import {
    listLauncherReleases,
    type LauncherRelease,
  } from "$lib/utils/github";
  import ChangelogRelease from "../../components/changelog/ChangelogRelease.svelte";
  import { _ } from "svelte-i18n";
  import { Alert, Button, Spinner, Toggle } from "flowbite-svelte";
  import IconRefresh from "~icons/mdi/refresh";

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

  {#each releases as release (release.version)}
    <ChangelogRelease
      {release}
      {showDependencyChanges}
      isCurrentVersion={release.version === launcherVersion}
    />
  {/each}
{:else}
  <Alert class="bg-slate-900 grow text-red-400">
    {$_("settings_changelog_noReleasesFound")}
  </Alert>
{/if}
