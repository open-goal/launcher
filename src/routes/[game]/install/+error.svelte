<script lang="ts">
  import { page } from "$app/stores";
  import { generateSupportPackage } from "$lib/rpc/support";
  import { Alert, Button } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
</script>

<!-- if the install fails for some reason we should end up here -->

<h1>{$page.status}: {$page.error?.message}</h1>

<Alert
  class="dark:bg-slate-900 flex-grow border-t-4 text-red-400"
  rounded={false}
>
  <span class="font-medium text-red-500"
    >{$_("setup_installationFailed")}
  </span><span class="text-white"> {$page.error?.message}</span>
</Alert>
<Button
  class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
  onclick={async () => await generateSupportPackage()}
  >{$_("setup_button_getSupportPackage")}</Button
>
