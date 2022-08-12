<script>
  import { SupportedGame } from "$lib/constants";

  import { extractTextures, getAllTexturePacks } from "$lib/rpc/commands";
  import { decompileFromFile } from "$lib/setup/setup";
  import { texturePackPrompt } from "$lib/textures/textures";
  import { removeDir, removeFile } from "@tauri-apps/api/fs";
  import { appDir, join } from "@tauri-apps/api/path";
  import Table from "./Table.svelte";

  $: texturePacks = [];
  $: disabled = false;

  function handleInputChanged({ value, checked }) {
    if (checked) {
      // add this texture pack to the array, reassign array to trigger dynamic rendering
      texturePacks.push(value);
      texturePacks = texturePacks;
    } else {
      // remove this texture pack from the array
      texturePacks.splice(texturePacks.indexOf(value), 1);
      texturePacks = texturePacks;
    }
    console.log(texturePacks);
  }

  async function handleCompileTextures() {
    disabled = true;
    // persist the installed texture packs in local storage

    // reset the texture_replacements folder to default (deleting the dir should work for this)
    try {
      const textureReplacementDir = await join(
        await appDir(),
        "data/texture_replacements"
      );
      await removeDir(textureReplacementDir, { recursive: true });
    } catch (err) {
      console.error(err);
    }

    try {
      // extract texture packs in (proper) order to texture_replacements (proper order: for overridding purposes)
      await extractTextures(texturePacks);
      // await decompile game (similar to GameControls function, maybe that function should be moved into a seperate file)
      await decompileFromFile(SupportedGame.Jak1);
      // should be ready to play (fingers crossed)
    } catch (err) {
      console.error(err);
    }
    disabled = false;
  }

  async function handleDeleteTexturePack(path) {
    // remove the texture pack from the array if it's in there
    // another option would be to disable the delete button if the input for this texturepack is checked
    await removeFile(path);
  }
</script>

<h1>Texture Packs</h1>

{#await getAllTexturePacks() then packs}
  <Table {packs} />
{/await}

<table>
  <tr>
    <th>Author</th>
    <th>Description</th>
    <th>Version</th>
    <th>Enable</th>
  </tr>
  {#await getAllTexturePacks() then packs}
    {#each packs as pack}
      <tr>
        <td>{pack[0].author}</td>
        <td>{pack[0].description}</td>
        <td>{pack[0].version}</td>
        <td
          ><input
            type="checkbox"
            checked={false}
            {disabled}
            value={pack[1]}
            on:change={(e) => handleInputChanged(e.target)}
          /></td
        >
      </tr>
    {/each}
  {/await}
</table>

<button class="btn" {disabled} on:click={() => texturePackPrompt()}
  >Add new texture pack</button
>
<button
  class="btn"
  disabled={texturePacks.length == 0 || disabled}
  on:click={() => handleCompileTextures()}>Compile Texture Changes</button
>

<!-- <Table data={packs} /> -->
<style>
  /* * {
    text-align: center;
  } */
</style>
