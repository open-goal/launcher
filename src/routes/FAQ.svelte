<script>
  import { Alert, Button } from "flowbite-svelte";
</script>

<div class="ml-20">
  <div class="flex flex-col h-5/6 p-8">
    <Alert color="dark" rounded={false}>
      <span class="text-lg font-medium text-blue-700 dark:text-blue-800"
        >Help & Support</span
      >
      <div slot="extra">
        <div class="mt-2 mb-4 text-sm text-blue-700 dark:text-blue-800">
          TODO ---- Fill this out with some COOL text about HELPING people WITH
          THE GAME.
        </div>
        <div class="flex flex-col gap-2">
          <Button
            href="https://github.com/open-goal/launcher/issues"
            target="_blank"
            rel="noreferrer noopener"
            size="md"
            outline
            color="blue">Report Launcher Issue</Button
          >
          <Button
            href="https://github.com/open-goal/jak-project/issues"
            target="_blank"
            rel="noreferrer noopener"
            size="md"
            outline
            color="blue">Report Game Issue</Button
          >
          <Button
            href="https://discord.gg/VZbXMHXzWv"
            target="_blank"
            rel="noreferrer noopener"
            size="md"
            outline
            color="blue">Discord</Button
          >
        </div>
      </div>
    </Alert>
  </div>
</div>
