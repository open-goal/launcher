import { Octokit } from "@octokit/rest";
import { throttling } from "@octokit/plugin-throttling";
import { retry } from "@octokit/plugin-retry";
import * as fs from "fs";

Octokit.plugin(throttling);
Octokit.plugin(retry);
const octokit = new Octokit({
  auth: process.env.GITHUB_TOKEN,
  userAgent: "open-goal/launcher",
  log: {
    debug: () => {},
    info: () => {},
    warn: console.warn,
    error: console.error,
  },
  throttle: {
    onRateLimit: (retryAfter, options) => {
      octokit.log.warn(
        `Request quota exhausted for request ${options.method} ${options.url}`,
      );

      // Retry twice after hitting a rate limit error, then give up
      if (options.request.retryCount <= 2) {
        console.log(`Retrying after ${retryAfter} seconds!`);
        return true;
      }
    },
    onAbuseLimit: (retryAfter, options) => {
      // does not retry, only logs a warning
      octokit.log.warn(
        `Abuse detected for request ${options.method} ${options.url}`,
      );
    },
  },
});

// Parse out changes from the markdown body
/*

## What's Changed
* jak1/speedruns: Some final touches for speedrunning in jak 1 by @open-goal in https://github.com/open-goal/jak-project/pull/1830

**Full Changelog**: https://github.com/open-goal/jak-project/compare/v0.1.28...v0.1.29

*/
// Releases take the following format, if there is no `What's Changed` then it's a superfluous release, no changes
function changesFromBody(releaseBody) {
  // Simple parsing to try and stay as little tied to the auto-format as possible
  // Iterate through the lines, grab all github pull request links
  let changes = [];
  const lines = releaseBody.split("\n");
  for (const line of lines) {
    // check if the string contains a pull request link
    if (!line.toLowerCase().match(/.*github.com\/[^/]*\/[^/]*\/pull.*/)) {
      continue;
    }
    // Parse out the critical parts of the line
    let description = [];
    let contributor = null;
    let pullRequestUrl = null;
    const words = line.replace(/^(\* )/, "").split(" ");
    let i = 0;
    while (i < words.length) {
      const word = words[i];
      if (word == "by") {
        // found contributor, the next word should be the contributor
        if (i + 1 < words.length) {
          contributor = words[i + 1].replace(/^(@)/, "");
          i += 2;
        }
        continue;
      }
      if (word == "in") {
        // found url, the next word should be the link
        if (i + 1 < words.length) {
          pullRequestUrl = words[i + 1];
          i += 2;
        }
        continue;
      }
      // Otherwise, just add it to the description
      description.push(word);
      i++;
    }
    description = description.join(" ");

    // Check if the pull request has already been added
    let duplicate = false;
    for (const entry of changes) {
      if (entry.pullRequestUrl === pullRequestUrl) {
        duplicate = true;
        break;
      }
    }
    if (!duplicate) {
      // Create the change
      changes.push({
        description: description,
        contributor: contributor,
        pullRequestUrl: pullRequestUrl,
      });
    }
  }
  return changes;
}

// Get all values from workflow
const releaseId = process.env.RELEASE_ID;

if (releaseId === undefined || releaseId === "") {
  console.log("You didn't provide RELEASE_ID");
  process.exit(1);
}

// Pull down the `launcher` release metadata
const { data: launcherRelease } = await octokit.rest.repos.getRelease({
  owner: "open-goal",
  repo: "launcher",
  release_id: releaseId,
});

if (launcherRelease === undefined) {
  console.log(`Couldn't find launcher release with id ${releaseId}`);
  process.exit(1);
}
console.log(launcherRelease);

// Get changes for the launcher
const launcherChanges = changesFromBody(launcherRelease.body);

// Retrieve application bundle signatures
const { data: releaseAssets } = await octokit.rest.repos.listReleaseAssets({
  owner: "open-goal",
  repo: "launcher",
  release_id: releaseId,
  per_page: 100,
});

let linuxSignature = "";
let windowsSignature = "";
let macosSignature = "";
for (var i = 0; i < releaseAssets.length; i++) {
  const asset = releaseAssets[i];
  if (asset.name.toLowerCase().endsWith("appimage.tar.gz.sig")) {
    const assetDownload = await octokit.rest.repos.getReleaseAsset({
      owner: "open-goal",
      repo: "launcher",
      asset_id: asset.id,
      headers: {
        Accept: "application/octet-stream",
      },
    });
    linuxSignature = Buffer.from(assetDownload.data).toString();
  }
  if (asset.name.toLowerCase().endsWith("msi.zip.sig")) {
    const assetDownload = await octokit.rest.repos.getReleaseAsset({
      owner: "open-goal",
      repo: "launcher",
      asset_id: asset.id,
      headers: {
        Accept: "application/octet-stream",
      },
    });
    windowsSignature = Buffer.from(assetDownload.data).toString();
  }
  if (asset.name.toLowerCase().endsWith("app.tar.gz.sig")) {
    const assetDownload = await octokit.rest.repos.getReleaseAsset({
      owner: "open-goal",
      repo: "launcher",
      asset_id: asset.id,
      headers: {
        Accept: "application/octet-stream",
      },
    });
    macosSignature = Buffer.from(assetDownload.data).toString();
  }
}

const releaseMeta = {
  name: launcherRelease.tag_name,
  notes: JSON.stringify({
    changes: launcherChanges,
  }),
  pub_date: launcherRelease.created_at,
  platforms: {
    "linux-x86_64": {
      signature: linuxSignature,
      url: `https://github.com/open-goal/launcher/releases/download/${
        launcherRelease.tag_name
      }/OpenGOAL-Launcher_${launcherRelease.tag_name.replace(
        "v",
        "",
      )}_amd64.AppImage.tar.gz`,
    },
    "windows-x86_64": {
      signature: windowsSignature,
      url: `https://github.com/open-goal/launcher/releases/download/${
        launcherRelease.tag_name
      }/OpenGOAL-Launcher_${launcherRelease.tag_name.replace(
        "v",
        "",
      )}_x64_en-US.msi.zip`,
    },
    "darwin-x86_64": {
      signature: macosSignature,
      url: `https://github.com/open-goal/launcher/releases/download/${launcherRelease.tag_name}/OpenGOAL-Launcher_x64.app.tar.gz`,
    },
  },
};
fs.writeFileSync(
  "./.tauri/latest-release-v2.json",
  JSON.stringify(releaseMeta, null, 2) + "\n",
);

// Publish the release
await octokit.rest.repos.updateRelease({
  owner: "open-goal",
  repo: "launcher",
  release_id: launcherRelease.id,
  draft: false,
});
