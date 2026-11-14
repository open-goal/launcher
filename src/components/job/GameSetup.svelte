<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { folderPrompt, isoPrompt } from "$lib/utils/file-dialogs";
  import { _ } from "svelte-i18n";
  import { navigate } from "/src/router";
  import { asJobType } from "$lib/job/jobs";

  import { route } from "/src/router";
  import { toSupportedGame } from "$lib/rpc/SupportedGame";
  import GameName from "../games/GameName.svelte";

  const activeGame = $derived(
    toSupportedGame((route.params as { game_name: string }).game_name)!,
  );

  async function install(viaFolder: boolean) {
    let sourcePath = undefined;
    if (viaFolder) {
      sourcePath = await folderPrompt($_("setup_prompt_selectFolderWithISO"));
    } else {
      sourcePath = await isoPrompt(
        $_("setup_prompt_ISOFileLabel"),
        $_("setup_prompt_selectISO"),
      );
    }
    if (sourcePath !== undefined) {
      navigate("/job/:job_type", {
        params: {
          job_type: asJobType("installGame"),
        },
        search: {
          activeGame: `${activeGame}`,
          sourcePath: sourcePath,
          returnTo: `/${activeGame}`,
        } as any,
      });
    }
  }
</script>

<div class="flex h-full p-5">
  <div class="flex flex-col items-end mt-auto ml-auto">
    <div class="pb-2 text-shadow-lg text-right">
      <GameName {activeGame}></GameName>
    </div>

    <Button
      class="min-w-[320px] font-semibold text-gray-200 h-12 text-2xl border-solid border border-[#1e3a66] rounded bg-[#13294b] hover:bg-[#183763] hover:border-[#2a4a7c] hover:text-white"
      onclick={async () => await install(false)}
      >{$_("setup_button_installViaISO")}</Button
    >
  </div>
</div>

<!-- TODO - disabled for now, needs fixes in the extractor -->
<!-- <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      onclick={async () => await install(true)}
      >Install via Folder</Button
    > -->
<!-- </div> -->
