<script type="ts">
  import { launcherConfig } from "$lib/config";

  import { onMount } from "svelte";

  let componentLoaded = false;

  let isAVXMet = false;
  let isOpenGLMet = false;

  onMount(async () => {
    isAVXMet = await launcherConfig.isAVXRequirementMet();
    isOpenGLMet = await launcherConfig.isOpenGLRequirementMet();
    componentLoaded = true;
  });
</script>

{#if componentLoaded}
  <div class="row">
    <p class="description">OpenGOAL Requires the following requirements:</p>
    <ul class="requirements-list">
      <li>
        {#if isAVXMet}
          ✅ CPU Supports <a
            class="help-link"
            target="_blank"
            href="https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX"
            >AVX</a
          >
        {:else}
          ❌ CPU Does NOT Support <a
            class="help-link"
            target="_blank"
            href="https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX"
            >AVX</a
          >
        {/if}
      </li>
      <li>
        {#if isOpenGLMet}
          ✅ GPU Supports <a
            class="help-link"
            target="_blank"
            href="https://en.wikipedia.org/wiki/OpenGL#OpenGL_4.3">OpenGL 4.3</a
          >
        {:else}
          ❌ GPU does NOT Support <a
            class="help-link"
            target="_blank"
            href="https://en.wikipedia.org/wiki/OpenGL#OpenGL_4.3">OpenGL 4.3</a
          >
        {/if}
      </li>
    </ul>
  </div>
{/if}

<style>
  .description {
    font-size: 1.5em;
    font-weight: 700;
  }

  .requirements-list {
    list-style: none;
    font-size: 1.25em;
  }

  .help-link {
    color: #ffc301;
  }
</style>
