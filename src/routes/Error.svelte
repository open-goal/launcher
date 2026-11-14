<script lang="ts">
  import { Alert, Button } from "flowbite-svelte";
  import { route } from "../router";
  import { generateSupportPackage } from "$lib/rpc/support";
  import IconPackageDown from "~icons/mdi/package-down";
  import { _ } from "svelte-i18n";

  const message = $derived((route.state as { message: string }).message);
</script>

<main
  class="flex flex-col min-h-full items-center justify-center bg-gray-50 dark:bg-[#141414]"
>
  <div class="w-full max-w-4xl space-y-4">
    <h1 class="text-3xl font-bold text-center text-gray-900 dark:text-white">
      Something went wrong!
    </h1>

    <Alert color="red" class="text-center">
      <span class="font-bold">Error:</span>
      {message}
    </Alert>

    <Button
      id="get-support-package"
      class="border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold w-full"
      onclick={async () => await generateSupportPackage()}
      ><IconPackageDown />&nbsp;{$_("setup_button_getSupportPackage")}
    </Button>
  </div>
</main>
