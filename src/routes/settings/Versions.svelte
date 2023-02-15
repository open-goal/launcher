<script>
  import { Alert, Button, Tabs, TabItem, Radio } from "flowbite-svelte";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";

  let officialVersion = "v0.1.31";
</script>

<Tabs style="pill">
  <TabItem open>
    <span slot="title">Official</span>
    <Table>
      <TableHead>
        <TableHeadCell class="sr-only" />
        <TableHeadCell>Version Number</TableHeadCell>
        <TableHeadCell>Date</TableHeadCell>
        <TableHeadCell>Github Link</TableHeadCell>
        <TableHeadCell>
          <span class="sr-only"> Download </span>
        </TableHeadCell>
      </TableHead>
      <TableBody class="divide-y">
        <TableBodyRow>
          <TableBodyCell class="!p-4">
            <Radio group={officialVersion} value="v0.1.31" />
          </TableBodyCell>
          <TableBodyCell>v0.1.31</TableBodyCell>
          <TableBodyCell>NOW</TableBodyCell>
          <TableBodyCell>TODO</TableBodyCell>
          <TableBodyCell>
            <a
              href="/tables"
              class="font-medium text-blue-600 hover:underline dark:text-blue-500"
            >
              Download
            </a>
          </TableBodyCell>
        </TableBodyRow>
        <TableBodyRow>
          <TableBodyCell class="!p-4">
            <Radio group={officialVersion} value="v0.1.30" disabled />
          </TableBodyCell>
          <TableBodyCell>v0.1.30</TableBodyCell>
          <TableBodyCell>NOW</TableBodyCell>
          <TableBodyCell>TODO</TableBodyCell>
          <TableBodyCell>
            <a
              href="/tables"
              class="font-medium text-blue-600 hover:underline dark:text-blue-500"
            >
              Download
            </a>
          </TableBodyCell>
        </TableBodyRow>
      </TableBody>
    </Table>
  </TabItem>
  <TabItem>
    <span slot="title">Unofficial</span>
    <p class="text-sm text-gray-500 dark:text-gray-400">
      <b>Dashboard:</b> Lorem ipsum dolor sit amet, consectetur adipiscing elit,
      sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
    </p>
  </TabItem>
  <TabItem>
    <span slot="title">Development</span>
    <p class="text-sm text-gray-500 dark:text-gray-400">
      <b>Settings:</b> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
      do eiusmod tempor incididunt ut labore et dolore magna aliqua.
    </p>
  </TabItem>
</Tabs>
