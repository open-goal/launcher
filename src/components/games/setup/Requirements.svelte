<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import { isAVXRequirementMet, isOpenGLRequirementMet } from "$lib/rpc/config";

  let isAVXMet = false;
  let isOpenGLMet = false;

  const dispatch = createEventDispatcher();

  onMount(async () => {
    isAVXMet = await isAVXRequirementMet(false);
    isOpenGLMet = await isOpenGLRequirementMet(false);
  });

  function alertColor(val: boolean | undefined) {
    if (val === undefined) {
      return "yellow";
    }
    return val ? "green" : "red";
  }
</script>

<div
  class="flex flex-col h-full justify-center items-center p-5 text-center gap-3"
>
  <h1 class="text-xl font-black mb-5 text-outline">
    Unfortunately, your system does not meet all the minimum requirements or we
    were unable to check them
  </h1>
  <Alert
    class="w-full text-start"
    accent
    rounded={false}
    color={alertColor(isAVXMet)}
  >
    {#if isAVXMet}
      <span class="font-bold">Your CPU supports AVX</span>
    {:else if isAVXMet === undefined}
      <span class="font-bold">Unable to verify if your CPU supports AVX</span>
    {:else}
      <span class="font-bold">Your CPU does not support AVX</span>
      <ul class="font-medium list-disc list-inside">
        <li>This cannot be fixed without upgrading to a newer CPU</li>
        <li>AVX support has been fairly standard since 2011</li>
        <li>
          <a
            class="font-bold text-blue-500"
            target="_blank"
            rel="noreferrer"
            href="https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX"
            >Click here for more information</a
          >
        </li>
      </ul>
    {/if}
  </Alert>
  <Alert
    class="w-full text-start"
    accent
    rounded={false}
    color={alertColor(isOpenGLMet)}
  >
    {#if isOpenGLMet}
      <span class="font-bold">Your GPU supports OpenGL 4.3</span>
    {:else if isOpenGLMet === undefined}
      <span class="font-bold"
        >Unable to verify if your GPU supports OpenGL 4.3</span
      >
    {:else}
      <span class="font-bold">Your GPU does not support OpenGL 4.3</span>
      <ul class="font-medium list-disc list-inside">
        <li>
          Lookup your GPU <a
            class="font-bold text-blue-500"
            target="_blank"
            rel="noreferrer"
            href="https://www.techpowerup.com/gpu-specs/">here</a
          > to see if it should be supported
        </li>
        <li>You can attempt to upgrade your GPU drivers</li>
        <li>
          Otherwise, you will need to upgrade your GPU, most GPUs since 2012
          support it
        </li>
      </ul>
    {/if}
  </Alert>
  <Button
    btnClass="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
    on:click={async () => {
      isAVXMet = await isAVXRequirementMet(true);
      isOpenGLMet = await isOpenGLRequirementMet(true);
      dispatch("change");
    }}>Re-check Requirements</Button
  >
</div>
