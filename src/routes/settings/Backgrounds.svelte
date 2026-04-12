<script>
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { onMount } from "svelte";
  import IconFolderOpen from "~icons/mdi/folder-open";
  // import IconDownload from "~icons/mdi/download";
  import { Button, Alert } from "flowbite-svelte";
  import { openPath } from "@tauri-apps/plugin-opener";
  import { exists, mkdir } from "@tauri-apps/plugin-fs";
  import { _ } from "svelte-i18n";

  let appBackgroundsDir = $state("");

  onMount(async () => {
    const appDataDirPath = await appDataDir();
    appBackgroundsDir = await join(appDataDirPath, "backgrounds");
    if (!(await exists(appBackgroundsDir))) {
      mkdir(appBackgroundsDir);
    }
  });
</script>

<div class="flex flex-col gap-2 mt-2">
  <Alert
    rounded={false}
    class="bg-white dark:bg-slate-900 border-t-4 text-red-400"
  >
    <p class="font-bold">{$_("settings_backgrounds_info_1")}</p>
    <ul class="ms-4 mt-1.5 list-inside list-disc font-semibold">
      <li>{$_("settings_backgrounds_info_2")}</li>
      <li>{$_("settings_backgrounds_info_3")}</li>
    </ul>
  </Alert>

  <div class="flex flex-col gap-2">
    <Button
      class="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-4 py-2"
      onclick={() => {
        openPath(appBackgroundsDir);
      }}
      ><IconFolderOpen /> &nbsp; {$_(
        "settings_backgrounds_button_openBackgroundsDir",
      )}</Button
    >
    <!-- <Button
      class="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-4 py-2"
      ><IconDownload /> &nbsp; {$_(
        "settings_backgrounds_button_downloadDefaultVideos",
      )}</Button
    > -->
  </div>
</div>
