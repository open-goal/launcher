<script lang="ts">
  import type { PageProps } from "./$types";
  import { locale as svelteLocale, _ } from "svelte-i18n";
  import LocaleSelector from "../../components/LocaleSelector.svelte";
  import ChooseInstallFolder from "../../components/ChooseInstallFolder.svelte";
  import { Button } from "flowbite-svelte";
  import { goto } from "$app/navigation";

  let { data }: PageProps = $props();
  const game = data.lastGame;
  let canContinue = $state(false); // build this out so it depends on locale $svelteLocale && $installDir
</script>

<div
  class="flex flex-col justify-center pb-24 w-screen h-screen bg-zinc-900 px-40 gap-4"
>
  <LocaleSelector />
  <ChooseInstallFolder />
  <!-- this continue button should only show if the user has selected a locale and install directory-->
  <Button
    disabled={!canContinue}
    onclick={() => goto(`/${game}`)}
    class="splash-button pointer-events-auto bg-orange-500 p-1 mt-1 text-black font-bold rounded hover:bg-orange-700"
    >Continue</Button
  >
</div>
