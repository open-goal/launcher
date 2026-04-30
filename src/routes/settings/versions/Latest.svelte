<script lang="ts">
  import { Button, Badge } from "flowbite-svelte";
  import type { ReleaseInfo } from "$lib/utils/github";
  import { versionState } from "/src/state/VersionState.svelte";
  import IconDeleteForever from "~icons/mdi/delete-forever";
  import IconDownload from "~icons/mdi/download";
  import IconCalendar from "~icons/mdi/CalendarMonth";
  import IconRefresh from "~icons/mdi/refresh";
  import IconGitHub from "~icons/mdi/github";
  import IconCheck from "~icons/mdi/CheckBold";
  import IconStar from "~icons/mdi/Star";
  import { _ } from "svelte-i18n";
  import { getLocale } from "$lib/rpc/config";
  import { onMount } from "svelte";

  let locale = $state("en-US");

  onMount(async () => {
    const res = await getLocale();
    if (res) locale = res;
  });

  let {
    latest,
    onRemoveVersion,
    onDownloadVersion,
    isPending,
  }: {
    latest: ReleaseInfo;
    onRemoveVersion: (version: string) => Promise<void>;
    onDownloadVersion: (version: string, downloadUrl: string) => Promise<void>;
    isPending: boolean;
  } = $props();

  const cleanDate = $derived.by(() => {
    if (!latest?.date) return "N/A";
    const date = new Date(latest.date);
    return date.toLocaleString(locale, {
      month: "long",
      day: "numeric",
      year: "numeric",
      timeZone: "UTC",
    });
  });

  const installed = $derived.by(() => {
    if (!latest?.version) return false;
    return latest.version === versionState.activeToolingVersion;
  });

  const handleRemove = async () => {
    await onRemoveVersion(latest.version);
  };

  const handleRedownload = async () => {
    if (latest.downloadUrl) {
      await onRemoveVersion(latest.version);
      await onDownloadVersion(latest.version, latest.downloadUrl);
    }
  };

  const handleDownload = async () => {
    if (!latest.downloadUrl) return;
    await onDownloadVersion(latest.version, latest.downloadUrl);
  };
</script>

<div
  class="flex items-center justify-between my-4 p-6 min-h-48 border border-green-500/40 rounded-md bg-green-800/15"
>
  <div class="flex flex-col gap-4">
    <Badge color="green" class="w-fit gap-1 tracking-wide">
      {#if installed}
        <IconCheck /> {$_("settings_versions_active_version")}
      {:else}
        <IconStar /> {$_("settings_versions_latest_release")}
      {/if}
    </Badge>

    <h1 class="text-4xl font-bold tracking-wide text-gray-200">
      {latest?.version}
    </h1>

    <p class="flex items-center gap-1 tracking-wide text-gray-300 text-sm">
      <IconCalendar /> Released {cleanDate}
    </p>

    <a
      href={latest.githubLink}
      target="_blank"
      rel="noreferrer"
      class="flex items-center gap-1 cursor-pointer tracking-wide text-gray-300 hover:text-white text-sm capitalize"
    >
      <IconGitHub />
      {$_("settings_versions_icon_githubRelease_altText")}
    </a>
  </div>

  <div class="flex flex-col items-end gap-2">
    {#if installed || latest.pendingAction}
      <div class="flex flex-col gap-4">
        <Button
          class="gap-1 capitalize text-md font-semibold rounded-sm bg-white/10 border border-white/15 hover:bg-white/15"
          onclick={handleRedownload}
          disabled={isPending}
        >
          <IconRefresh
            class="text-xl"
            aria-label={$_("settings_versions_icon_redownloadVersion_altText")}
          />
          {$_("settings_versions_icon_redownloadVersion_altText")}
        </Button>

        <Button
          class="gap-1 capitalize text-md font-semibold rounded-sm border border-white/15 bg-red-600/60 hover:bg-red-700"
          onclick={handleRemove}
          disabled={isPending}
        >
          <IconDeleteForever
            class="text-xl"
            color="white"
            aria-label={$_("settings_versions_icon_removeVersion_altText")}
          />{$_("settings_versions_icon_removeVersion_altText")}
        </Button>
      </div>
    {:else}
      <Button
        class="px-8 py-3 text-lg gap-1 font-semibold rounded-sm text-gray-200 bg-green-500 hover:bg-green-600"
        onclick={handleDownload}
        disabled={isPending}
      >
        <IconDownload />
        <span class="capitalize">
          {$_("settings_versions_icon_downloadVersion_altText")}
        </span>
        {latest?.version}
      </Button>
    {/if}
  </div>
</div>
