<script>
  import { Button } from "flowbite-svelte";
</script>

<div class="flex flex-col h-full justify-center items-center p-5 text-center">
  <h1 class="text-2xl font-black mb-5 text-outline">
    No Tooling Version Configured!
  </h1>
  <p class="mb-10">
    Head over to the following settings page to download the latest release
  </p>
  <Button
    btnClass="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
    href="/settings/versions">Set Version</Button
  >
</div>
