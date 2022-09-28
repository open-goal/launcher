<script>
  import { Alert, Button } from "flowbite-svelte";
</script>

<div class="ml-20">
  <div class="flex flex-col h-5/6 p-8">
    <Alert color="dark" rounded={false} accent>
      <span class="text-lg font-medium text-black-900 dark:text-blue-800"
        >Help & Support</span
      >
      <div slot="extra">
        <div class="mt-2 mb-4 text-sm text-blue-700 dark:text-blue-800">
          Please use the discord search feature to see if your question has
          already been answered
        </div>
        <div class="flex flex-col gap-2">
          <Button
            class="!rounded-none"
            href="https://github.com/open-goal/launcher/issues"
            target="_blank"
            rel="noreferrer noopener"
            size="md"
            outline
            color="blue">Report Launcher Issue</Button
          >
          <Button
            class="!rounded-none"
            href="https://github.com/open-goal/jak-project/issues"
            target="_blank"
            rel="noreferrer noopener"
            size="md"
            outline
            color="blue">Report Game Issue</Button
          >
          <Button
            class="!rounded-none"
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
