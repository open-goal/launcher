<script lang="ts">
  import type { PageProps } from "./$types";
  import { _ } from "svelte-i18n";
  import IconRefresh from "~icons/mdi/refresh";
  import IconGitHub from "~icons/mdi/github";
  import IconDownload from "~icons/mdi/download";
  import IconDeleteForever from "~icons/mdi/delete-forever";
  import {
    Button,
    Radio,
    Spinner,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";
  import { downloadOfficialVersion, removeVersion } from "$lib/rpc/versions";
  import { saveActiveVersionChange } from "$lib/rpc/config";
  import { invalidateAll } from "$app/navigation";
  import type { ReleaseInfo } from "$lib/utils/github";
  import { toastStore } from "$lib/stores/ToastStore";

  let { data }: PageProps = $props();
  const config = $derived(data.config);
  const official = $derived(data.official);
  const installed = $derived(data.installed);

  const installedSet = $derived(new Set(installed));
  const isDownloaded = (r: ReleaseInfo) => installedSet.has(r.version);

  const pending = $state<Record<string, boolean>>({});
  const isPending = (r: ReleaseInfo) => !!pending[r.version];
  const anyPending = $derived(Object.values(pending).some(Boolean));

  async function onDownload(release) {
    pending[release.version] = true;
    await downloadOfficialVersion(release);
    await saveActiveVersionChange(release);
    toastStore.makeToast($_("toasts_savedToolingVersion"), "info");
    await invalidateAll();
    pending[release.version] = false;
  }

  async function onRemove(release) {
    pending[release.version] = true;
    await removeVersion(release);
    await invalidateAll();
    pending[release.version] = false;
  }

  let active = $derived(config.activeVersion);
</script>

{#each official as release (release.version)}
  <TableBodyRow class="bg-zinc-900">
    <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
      {#if isDownloaded(release)}
        <Radio
          bind:group={active}
          value={release.version}
          onchange={async () => {
            await saveActiveVersionChange(release);
            await invalidateAll();
          }}
          disabled={!isDownloaded(release)}
          class="disabled:cursor-not-allowed p-0"
        />
      {/if}
    </TableBodyCell>
    <TableBodyCell
      class="px-6 py-2 whitespace-nowrap font-medium"
      style="line-height: 0;"
    >
      <Button
        class="py-0 bg-transparent focus:ring-0 disabled:opacity-50"
        disabled={anyPending}
        onclick={async () => {
          if (isDownloaded(release)) {
            await onRemove(release);
          } else {
            await onDownload(release);
          }
        }}
      >
        {#if isDownloaded(release)}
          <IconDeleteForever
            class="text-xl"
            color="red"
            aria-label={$_("settings_versions_icon_removeVersion_altText")}
          />
        {:else if isPending(release)}
          <Spinner color="yellow" size="6" />
        {:else}
          <IconDownload
            class="text-xl"
            color="#00d500"
            aria-label={$_("settings_versions_icon_downloadVersion_altText")}
          />
        {/if}
      </Button>
      {#if isDownloaded(release)}
        <Button
          class="py-0 bg-transparent focus:ring-0 disabled:opacity-50"
          disabled={anyPending}
          onclick={async () => {
            await onDownload(release);
          }}
        >
          {#if isPending(release)}
            <Spinner color="yellow" size="6" />
          {:else}
            <IconRefresh
              class="text-xl"
              aria-label={$_(
                "settings_versions_icon_redownloadVersion_altText",
              )}
            />
          {/if}
        </Button>
      {/if}
    </TableBodyCell>
    <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
      {release.version}
    </TableBodyCell>
    <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
      {new Date(release.date).toLocaleDateString()}
    </TableBodyCell>
    <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
      <a href={release.githubLink} target="_blank" rel="noreferrer">
        <IconGitHub
          class="text-xl"
          aria-label={$_("settings_versions_icon_githubRelease_altText")}
        />
      </a>
    </TableBodyCell>
  </TableBodyRow>
{/each}
