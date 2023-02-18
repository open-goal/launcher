<script lang="ts">
  import { onMount } from "svelte";
  import { extractTextures, getAllTexturePacks } from "$lib/rpc/commands";
  import { texturePackPrompt } from "$lib/textures/textures";
  import { appDir, join } from "@tauri-apps/api/path";
  import { removeDir, removeFile } from "@tauri-apps/api/fs";
  import { SupportedGame } from "$lib/constants";
  import { confirm } from "@tauri-apps/api/dialog";
  import {
    Alert,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Checkbox,
    ButtonGroup,
    Button,
    Tooltip,
  } from "flowbite-svelte";

  interface TexturePack {
    author: String;
    description: String;
    version: String;
    path: String;
  }

  let packs: Array<TexturePack> = [];
  let selectedTexturePacks: string[] = [];
  $: disabled = false;

  // TODO - deferring this work

  onMount(async () => {
    // packs = await getAllTexturePacks();
  });

  async function handleSelectedPacks(pack) {
    if (!selectedTexturePacks.find((packs) => packs === pack)) {
      // add pack to be compiled
      selectedTexturePacks.push(pack);
      selectedTexturePacks = selectedTexturePacks;
    } else {
      // remove pack from to be compiled
      selectedTexturePacks = selectedTexturePacks.filter(
        (packs) => packs !== pack
      );
    }
  }

  async function handleAddTexturePack() {
    try {
      await texturePackPrompt();
      // packs = await getAllTexturePacks();
    } catch (error) {
      console.error(error);
    }
  }

  async function handleDeleteTexturePack() {
    disabled = true;
    // TODO: Update this confirmation to an in-app modal
    const confirmed = await confirm(
      "Are you sure you would like to delete this pack?",
      {
        title: "Texture Packs",
        type: "warning",
      }
    );

    if (confirmed) {
      for (let pack of selectedTexturePacks) {
        console.log("Deleting texture pack: ", pack);
        try {
          // delete the file from the texture_zips directory
          await removeFile(pack);
          // delete the relative object from the packs array to update the table
          packs = packs.filter((obj) => {
            return obj.path !== pack;
          });
          // empty the selectedTexturePacks array
          selectedTexturePacks = [];
        } catch (err) {
          console.error(err);
        }
      }
    }
    disabled = false;
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
      await extractTextures(selectedTexturePacks);
      // await decompile game (similar to GameControls function, maybe that function should be moved into a seperate file)
      // await decompileFromFile(SupportedGame.Jak1);
      // should be ready to play (fingers crossed)
    } catch (err) {
      console.error(err);
    }
    disabled = false;
  }
</script>

<div class="ml-20">
  <div class="flex flex-col h-[560px] max-h-[560px] p-8 gap-2">
    TODO
    <!-- {#if packs && packs.length > 0}
      <Table hoverable={true}>
        <TableHead>
          <TableHeadCell class="!p-4" />
          <TableHeadCell>Author</TableHeadCell>
          <TableHeadCell>Description</TableHeadCell>
          <TableHeadCell>Version</TableHeadCell>
        </TableHead>
        <TableBody class="divide-y">
          {#each packs as pack}
            <TableBodyRow id={pack.path}>
              <TableBodyCell class="!p-4">
                <Checkbox
                  on:click={() => handleSelectedPacks(pack.path)}
                  {disabled}
                />
              </TableBodyCell>
              <TableBodyCell>{pack.author}</TableBodyCell>
              <TableBodyCell tdClass="overflow-clip"
                >{pack.description}</TableBodyCell
              >
              <TableBodyCell>{pack.version}</TableBodyCell>
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    {:else}
      <Alert color="yellow" accent rounded={false}
        >No Texture Packs Installed, get started by adding a pack below.</Alert
      >
    {/if}
    <ButtonGroup class="ml-auto mt-auto">
      <Button
        size="md"
        class="!rounded-none"
        color="green"
        {disabled}
        on:click={handleAddTexturePack}>Add Pack</Button
      >
      <Tooltip rounded={false}>Add a new pack to the table</Tooltip>
      <Button
        class="!rounded-none"
        color="red"
        disabled={disabled || selectedTexturePacks.length === 0}
        on:click={handleDeleteTexturePack}>Delete Pack</Button
      >
      <Tooltip rounded={false}>Delete selected pack(s) from the table</Tooltip>
      <Button
        class="!rounded-none"
        color="dark"
        {disabled}
        on:click={async () => await handleCompileTextures()}
        >Compile Changes</Button
      >
      <Tooltip rounded={false}
        >Compile the game with the selected packs in the order they were
        selected</Tooltip
      >
    </ButtonGroup> -->
  </div>
</div>
