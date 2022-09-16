<script>
  import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
  import { relaunch } from "@tauri-apps/api/process";
  import { Alert, Button } from "flowbite-svelte";

  let disabled = false;

  async function updateHandler() {
    disabled = true;
    await installUpdate();
    await relaunch();
  }
</script>

<div class="ml-20">
  <div class="flex flex-col h-5/6 p-8">
    <!-- take this check update out of here and move this logic somewhere else, use a global store for this info -->
    {#await checkUpdate() then { shouldUpdate, manifest }}
      {#if shouldUpdate}
        <Alert color="dark" rounded={false} accent>
          <span class="text-2xl font-bold text-black-900">Update Available</span
          >
          <div slot="extra">
            <div class="mt-2 mb-4 text-lg text-blue-700">
              <p>{manifest.body}</p>
            </div>
            <div class="flex flex-col">
              <Button
                class="!rounded-none !bg-[#222222] border-none !text-white hover:!text-yellow-300 !text-2xl"
                on:click={async () => await updateHandler()}
                {disabled}>Update</Button
              >
            </div>
          </div>
        </Alert>
      {:else}
        <p>You're Up to Date!</p>
      {/if}
    {/await}
  </div>
</div>
