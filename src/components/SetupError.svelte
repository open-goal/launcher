<script>
  export let errorMsg;

  // function handleError(output) {
  //   if (output.code === 0) {
  //     return;
  //   }
  //   switch (output.code) {
  //     case 4000:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `can't locate ELF in ISO's contents`,
  //           description: "unable to determine the version of the game",
  //         },
  //       ];
  //       break;
  //     case 4001:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `Unsupported serial`,
  //           description:
  //             "ISO containing an unsupported game serial or version was provided",
  //         },
  //       ];
  //       break;
  //     case 4002:
  //     case 4010:
  //     case 4011:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `Unsupported game version`,
  //           description:
  //             "ISO contains files that are for an unsupported version, were modified from the original, or is an incomplete dump",
  //         },
  //       ];
  //       break;
  //     case 4020:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `Unexpected Extraction Result`,
  //           description:
  //             "The extracted ISO's contents were not as expected, installation cannot proceed",
  //         },
  //       ];
  //       break;
  //     default:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `Unexpected Error Code ${output.code}`,
  //           description:
  //             "An unexpected error occurred during installation, check logs",
  //         },
  //       ];
  //   }
  // }
</script>

<section>
  <h2>Error: {errorMsg}</h2>
</section>
