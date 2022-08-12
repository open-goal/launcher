<script>
  import { DataTable } from "carbon-components-svelte";

  export let packs;
  let selectedRowIds = [];

  $: console.log("selectedRowIds", selectedRowIds);
</script>

<DataTable
  selectable
  bind:selectedRowIds
  headers={[
    { key: "author", value: "Author" },
    { key: "description", value: "Description" },
    { key: "version", value: "Version" },
  ]}
  rows={packs.map((pack, i) => ({
    id: i,
    author: pack[0].author,
    description: pack[0].description,
    version: pack[0].version,
    value: pack[1],
  }))}
/>
