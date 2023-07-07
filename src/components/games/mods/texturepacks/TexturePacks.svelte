<!--
  - go back to game
  - add button
  - verify if texture packs are still installed (ie user manually deletes them)
  - save changes (re-compiles game)
  - per row
    - enable checkbox
    - remove button
    - buttons to change order up/down
    - conflict warnings (if this texture pack has any that are shared with others)
    - info about it
 -->

<!-- {
  "name": "My Texture Pack",
  "version": "1.0.0",
  "description": "My Texture Pack",
  "author": "My Name",
  "tags": ["something", "something-else"],
  "releaseDate": "2022-01-01"
} -->

<!-- TODO - collecting rating metrics / number of users might be cool (same for mods) -->

<script lang="ts">
  import type { SupportedGame } from "$lib/constants";
  import Icon from "@iconify/svelte";
  import { Accordion, AccordionItem, Alert, Badge, Button, Card, Checkbox, Tooltip } from "flowbite-svelte";
  import { onMount } from "svelte";

  export let activeGame: SupportedGame;

  onMount(async () => {});
</script>

<div class="flex flex-col mt-auto">
  <div class="flex flex-row gap-2">
    <!-- Placeholder image -->
    <Card
      img="https://flowbite-svelte.com/images/image-1.webp"
      horizontal
    >
      <div class="flex flex-row mt-auto items-center">
        <h2 class="text-xl font-bold tracking-tight text-white">
          My Texture Pack
          <span class="text-xs text-gray-500">1.0.0</span>
        </h2>
        <p
          class="flex flex-grow justify-end items-end text-xs ml-5 text-gray-500 leading-normal"
        >
          by My Name
        </p>
      </div>
      <p class="mb-5 font-normal text-gray-400 leading-tight">
        My Texture Pack description
      </p>
      <!-- TODO - give tags colors -->
      <Badge border color="green">realism</Badge>
      <Badge border color="green">parody</Badge>
      <p class="mt-2 font-bold text-gray-500 text-xs">Released - 2022-01-01</p>
      <!-- Buttons -->
      <div class="mt-2 flex flex-row gap-2">
        <Checkbox checked class="m-0" style="width:1.75rem;height:1.75rem;"/>
        <Button outline 
          class="!p-1.5 rounded-md border-green-500 text-green-500 hover:bg-green-600"
        >
          <Icon icon="material-symbols:arrow-upward" width="15" height="15" />
        </Button>
        <Button outline 
          class="!p-1.5 rounded-md border-green-500 text-green-500 hover:bg-green-600"
        >
          <Icon icon="material-symbols:arrow-downward" width="15" height="15" />
        </Button>
        <Button outline
          class="!p-1.5 rounded-md border-green-500 text-green-500 hover:bg-green-600"
        >
          <Icon icon="material-symbols:delete" width="15" height="15" />
        </Button>
      </div>
      <Accordion flush class="mt-2">
        <AccordionItem paddingFlush="p-2">
          <span slot="header" class="flex gap-2 text-yellow-300 text-sm">
            <svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
            <span> Conflicts Detected!</span>
          </span>
          <p class="mb-2 text-gray-500 dark:text-gray-400">Lorem ipsum dolor sit amet, consectetur adipisicing elit. Illo ab necessitatibus sint explicabo...</p>
        </AccordionItem>
      </Accordion>
    </Card>
  </div>
</div>
