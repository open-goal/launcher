<script type="ts">
  import { launcherConfig } from "$lib/config";
  import { onMount } from "svelte";
  import { Alert } from "flowbite-svelte";

  let isAVXMet = false;
  let isOpenGLMet = false;

  onMount(async () => {
    isAVXMet = await launcherConfig.isAVXRequirementMet();
    isOpenGLMet = await launcherConfig.isOpenGLRequirementMet();
  });
</script>

<div class="space-y-2">
  <Alert
    class="flex justify-center rounded-none"
    accent
    color={isAVXMet ? "green" : "red"}
  >
    <span class="font-bold"
      >Your CPU {isAVXMet ? "supports" : "doesn't support"} AVX</span
    >
  </Alert>

  <Alert
    class="flex flex-col rounded-none"
    accent
    color={isOpenGLMet ? "green" : "red"}
  >
    <span class="font-bold"
      >Your GPU {isOpenGLMet ? "supports" : "doesn't support"} OpenGL 4.3</span
    >
    {#if !isOpenGLMet}
      <ul class="mt-0 ml-8 list-disc list-inside">
        <li>Please try updating your graphics card drivers</li>
        <li>
          If your drivers are up to date, you are unable to play due to a
          hardware limitation
        </li>
      </ul>
    {/if}
  </Alert>
</div>
