<script lang="ts">
  import { _ } from "svelte-i18n";
  import GameJob from "../old_components/games/job/GameJob.svelte";
  import GameToolsNotSet from "../old_components/games/GameToolsNotSet.svelte";
  import type { Job } from "$lib/utils/jobs";

  let modName: string | undefined = undefined;
  let modSource: string | undefined = undefined;
  let modVersionToInstall: string = "";
  let modDownloadUrlToInstall: string = "";

  let gameJobToRun: Job | undefined = undefined;

  async function runGameJob(event: any) {
    gameJobToRun = event.detail.type;
    if (gameJobToRun === "installModExternal") {
      modDownloadUrlToInstall = event.detail.modDownloadUrl;
      modVersionToInstall = event.detail.modVersion;
    }
  }
</script>

<div class="flex flex-col h-full p-5">
  {#if activeVersionName === null}
    <GameToolsNotSet />
  {:else if gameJobToRun !== undefined}
    <GameJob
      jobType={gameJobToRun}
      modSourceName={modSource}
      modDownloadUrl={modDownloadUrlToInstall}
      modVersion={modVersionToInstall}
      {modName}
    />
  {/if}
</div>
