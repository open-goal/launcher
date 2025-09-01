<script lang="ts">
  import type { PageProps } from "./$types";
  import { Button } from "flowbite-svelte";
  import { _ } from "svelte-i18n";

  let { data }: PageProps = $props();
</script>

<div class="flex flex-col h-full justify-center items-center p-4 text-center">
  <h1 class="text-2xl font-black mb-5 text-outline">
    {$_("gameControls_toolingTooOld_header")}
  </h1>
  <p class="mb-10">
    {$_("gameControls_toolingTooOld_subheader")}
  </p>
  <Button
    class="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
    href="/settings/versions"
    >{$_("gameControls_toolingTooOld_button_setVersion")}</Button
  >
</div>
