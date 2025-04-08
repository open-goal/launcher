import { Octokit } from "@octokit/rest";
import { throttling } from "@octokit/plugin-throttling";
import { retry } from "@octokit/plugin-retry";
import * as fs from "fs";

const owner = process.env.OWNER;
const repo = process.env.REPO;

Octokit.plugin(throttling);
Octokit.plugin(retry);
const octokit = new Octokit({
  auth: process.env.GITHUB_TOKEN,
  userAgent: `${owner}/${repo}`,
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
  owner: owner,
  repo: repo,
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
  owner: owner,
  repo: repo,
  release_id: releaseId,
  per_page: 100,
});

let jsonOutput = undefined;

for (var i = 0; i < releaseAssets.length; i++) {
  const asset = releaseAssets[i];
  if (asset.name.toLowerCase() === "latest.json") {
    const assetDownload = await octokit.rest.repos.getReleaseAsset({
      owner: owner,
      repo: repo,
      asset_id: asset.id,
      headers: {
        Accept: "application/octet-stream",
      },
    });
    jsonOutput = JSON.parse(Buffer.from(assetDownload.data).toString());
  }
}

// Make the download links idempotent instead of using 'latest' which is a moving target
// if the asset doesn't contain the version number (the macOS ones don't)
//
// - /releases/latest/download/ replace with /releases/download/v${version}/
const currentVersion = jsonOutput.version;
const replacementDownloadSubstring = `/releases/download/v${currentVersion}/`;
for (const [key, value] of Object.entries(jsonOutput.platforms)) {
  jsonOutput.platforms[key].url = value.url.replace(
    "/releases/latest/download/",
    replacementDownloadSubstring,
  );
}

if (jsonOutput === undefined) {
  console.log(`Didn't find 'latest.json' asset in release`);
  process.exit(1);
}

jsonOutput.notes = JSON.stringify({
  changes: launcherChanges,
});

fs.writeFileSync(
  "./.tauri/latest-release-v2.json",
  JSON.stringify(jsonOutput, null, 2) + "\n",
);

// Publish the release
await octokit.rest.repos.updateRelease({
  owner: owner,
  repo: repo,
  release_id: launcherRelease.id,
  draft: false,
});
