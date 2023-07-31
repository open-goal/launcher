// Verifies that a release has all the expected assets
// essentially a simple sanity check prior to publishing a release
//
// TODO - make this an action

import { Octokit } from "@octokit/rest";
import { throttling } from "@octokit/plugin-throttling";
import { retry } from "@octokit/plugin-retry";

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

// Get all values from workflow
const releaseId = process.env.RELEASE_ID;

if (releaseId === undefined || releaseId === "") {
  console.log("You didn't provide RELEASE_ID");
  process.exit(1);
}

// Pull down the `launcher` release metadata
const assets = await octokit.rest.repos.listReleaseAssets({
  owner: "open-goal",
  repo: "launcher",
  release_id: releaseId,
  per_page: 100,
});
console.log(assets);

let expectedAssetNameRegexes = process.env.EXPECTED_ASSET_NAME_REGEXES;
if (expectedAssetNameRegexes === undefined || expectedAssetNameRegexes === "") {
  console.log("You didn't provide EXPECTED_ASSET_NAME_REGEXES");
  process.exit(1);
}
expectedAssetNameRegexes = JSON.parse(expectedAssetNameRegexes);
let missingAsset = false;
for (const expectedAssetRegex of expectedAssetNameRegexes) {
  let assetMatchFound = false;
  for (const asset of assets.data) {
    if (new RegExp(expectedAssetRegex).test(asset.name)) {
      assetMatchFound = true;
      break;
    }
  }
  if (!assetMatchFound) {
    console.log(`No assets matched ${expectedAssetRegex}`);
    missingAsset = true;
  }
}

if (missingAsset) {
  process.exit(1);
}
