<script>
  import { openDir } from "$lib/commands";
  import { appDir } from "@tauri-apps/api/path";
</script>

<div>
  <h1>TODO!</h1>
  {#await appDir() then directory}
    <button class="btn" on:click={() => openDir(directory)}
      >Open App Directory</button
    >
    <p>{directory}</p>
  {/await}
</div>
