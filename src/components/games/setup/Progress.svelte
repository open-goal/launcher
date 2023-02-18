<script lang="ts">
  import { InstallationProgress } from "$lib/stores/AppStore";
  import Icon from "@iconify/svelte";
  $: progress = $InstallationProgress;

  // NOTE - useful for debugging:
  // let progress = {
  //   currentStep: 0,
  //   steps: [
  //     {
  //       status: "success",
  //     },
  //     {
  //       status: "pending",
  //     },
  //     {
  //       status: "queued",
  //     },
  //     {
  //       status: "queued",
  //     },
  //   ],
  // };

  const iconContainerStyle =
    "w-10 h-10 mx-auto border-solid border-2 border-slate-800 bg-slate-900 rounded-full text-lg text-white flex justify-center items-center";
  const stepLabelStyle = "text-center text-outline font-semibold";
  const progressBarContainerStyle =
    "w-full rounded items-center align-middle align-center flex-1";

  // TODO - this pattern indicates these should probably be their own components...
  function progressIcon(currentStatus: string) {
    if (currentStatus === "success") {
      return "material-symbols:check";
    } else if (currentStatus === "pending") {
      return "mdi:dots-horizontal";
    } else if (currentStatus === "failed") {
      return "mdi:stop-alert";
    }
    return "mdi:hourglass";
  }

  function progressIconStyle(currentStatus: string) {
    let style = "";
    if (currentStatus === "pending") {
      style += " animate-pulse";
    }
    return style;
  }

  function progressIconColor(currentStatus: string) {
    if (currentStatus === "success") {
      return "#22c55e";
    } else if (currentStatus === "pending") {
      return "#facc15";
    } else if (currentStatus === "failed") {
      return "#ef4444";
    }
    return "#737373";
  }

  function progressBarStyle(currentStatus: string) {
    let style = "w-full py-1 rounded";
    if (currentStatus === "success") {
      style += " bg-green-500";
    } else if (currentStatus === "pending") {
      style += " bg-yellow-400 animate-pulse";
    } else if (currentStatus === "queued") {
      style += " bg-slate-900";
    } else if (currentStatus === "failed") {
      style += " bg-red-500";
    }
    return style;
  }
</script>

<!-- ripped this component from online: https://tailwindcomponents.com/component/wizard-steps-bar-with-tailwind-css -->

<div class="w-full py-6">
  <div class="flex">
    <div class="w-1/4">
      <div class="relative mb-2">
        <!-- EXTRACTING AND VERIFYING -->
        <div class={iconContainerStyle}>
          <Icon
            class={progressIconStyle(progress.steps[0].status)}
            icon={progressIcon(progress.steps[0].status)}
            color={progressIconColor(progress.steps[0].status)}
            width={28}
            height={28}
          />
        </div>
      </div>
      <div class={stepLabelStyle}>
        Extracting<br />and Verifying
      </div>
    </div>
    <div class="w-1/4">
      <div class="relative mb-2">
        <div
          class="absolute flex align-center items-center align-middle content-center"
          style="width: calc(100% - 2.5rem - 1rem); top: 50%; transform: translate(-50%, -50%)"
        >
          <div class={progressBarContainerStyle}>
            <div class={progressBarStyle(progress.steps[0].status)} />
          </div>
        </div>
        <!-- DECOMPILING -->
        <div class={iconContainerStyle}>
          <Icon
            class={progressIconStyle(progress.steps[1].status)}
            icon={progressIcon(progress.steps[1].status)}
            color={progressIconColor(progress.steps[1].status)}
            width={28}
            height={28}
          />
        </div>
      </div>
      <div class={stepLabelStyle}>Decompiling</div>
    </div>
    <div class="w-1/4">
      <div class="relative mb-2">
        <div
          class="absolute flex align-center items-center align-middle content-center"
          style="width: calc(100% - 2.5rem - 1rem); top: 50%; transform: translate(-50%, -50%)"
        >
          <div class={progressBarContainerStyle}>
            <div class={progressBarStyle(progress.steps[1].status)} />
          </div>
        </div>
        <!-- COMPILING -->
        <div class={iconContainerStyle}>
          <Icon
            class={progressIconStyle(progress.steps[2].status)}
            icon={progressIcon(progress.steps[2].status)}
            color={progressIconColor(progress.steps[2].status)}
            width={28}
            height={28}
          />
        </div>
      </div>
      <div class={stepLabelStyle}>Compiling</div>
    </div>
    <div class="w-1/4">
      <div class="relative mb-2">
        <div
          class="absolute flex align-center items-center align-middle content-center"
          style="width: calc(100% - 2.5rem - 1rem); top: 50%; transform: translate(-50%, -50%)"
        >
          <div class={progressBarContainerStyle}>
            <div class={progressBarStyle(progress.steps[2].status)} />
          </div>
        </div>
        <!-- READY -->
        <div class={iconContainerStyle}>
          <Icon
            class={progressIconStyle(progress.steps[3].status)}
            icon={progressIcon(progress.steps[3].status)}
            color={progressIconColor(progress.steps[3].status)}
            width={28}
            height={28}
          />
        </div>
      </div>
      <div class={stepLabelStyle}>Done</div>
    </div>
  </div>
</div>
