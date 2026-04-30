<script>
  import { onMount } from "svelte";
  import {
    getInstallationDirectory,
    setInstallationDirectory,
    setLocale,
  } from "$lib/rpc/config";
  import { locale as svelteLocale, _ } from "svelte-i18n";
  import LocaleSelector from "../components/startup/LocaleSelector.svelte";
  import ChooseInstallFolder from "../components/startup/ChooseInstallFolder.svelte";
  import { navigate } from "../router";
  import { Button } from "flowbite-svelte";

  let locale = $state("");
  let installDir = $state("");

  onMount(async () => {
    await svelteLocale.set("en-US");
    installDir = (await getInstallationDirectory()) || "";
  });

  $effect(() => {
    if (!locale) return;
    svelteLocale.set(locale);
    setLocale(locale);
  });

  $effect(() => {
    if (!installDir) return;
    setInstallationDirectory(installDir);
  });
</script>

<div class="flex flex-col items-center justify-center h-full bg-[#141414]">
  <div class="w-full max-w-[90vw] space-y-8">
    <LocaleSelector bind:locale />

    <ChooseInstallFolder bind:installDir />

    <Button
      class="w-full rounded-lg border border-orange-500 bg-orange-500 px-4 text-black font-semibold transition hover:border-orange-400 hover:bg-orange-400 disabled:border-zinc-700 disabled:bg-zinc-800 disabled:text-zinc-500"
      disabled={!locale || !installDir}
      onclick={() =>
        navigate("/:game_name/", { params: { game_name: "jak1" } })}
      >{$_("startup_button_finishSetup")}
    </Button>
  </div>
</div>
