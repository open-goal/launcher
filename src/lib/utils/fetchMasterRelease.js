const { downloadRelease } = require("@terascope/fetch-github-release");
const { app } = require("electron");

const user = "xTVaser";
// const user = 'open-goal';
const repo = "jak-project";
const outputdir = app.getPath("userData") + "/release";
const leaveZipped = false;
const disableLogging = false;

// Define a function to filter releases.
function filterRelease(release) {
  // Filter out prereleases.
  return release.prerelease === false;
}

// Define a function to filter assets.
function filterAsset(asset) {
  // filter the assets based on the user OS
  switch (process.platform) {
    case "win32":
      return asset.name.includes("windows");
    case "darwin":
      return asset.name.includes("mac");
    case "linux":
      return asset.name.includes("linux");
    default:
      return asset.name.includes("windows");
  }
}

function fetchMasterRelease() {
  downloadRelease(
    user,
    repo,
    outputdir,
    filterRelease,
    filterAsset,
    leaveZipped,
    disableLogging
  )
    .then(function () {
      console.log("Master release downloaded successfully!");
      app.emit("console", "Master release downloaded successfully!");
    })
    .catch(function (err) {
      console.error(err.message);
      app.emit("console", err.message);
    });
}

module.exports = {
  fetchMasterRelease,
};
