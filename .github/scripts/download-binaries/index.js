import { Octokit } from "@octokit/rest";
import { throttling } from "@octokit/plugin-throttling";
import { retry } from "@octokit/plugin-retry";
import * as fs from "fs";
import * as path from "path";
import { exit } from "process";

Octokit.plugin(throttling);
Octokit.plugin(retry);
const octokit = new Octokit({
  auth: process.env.GITHUB_TOKEN,
  userAgent: "open-goal/jak-project",
  log: {
    debug: () => {},
    info: () => {},
    warn: console.warn,
    error: console.error,
  },
  throttle: {
    onRateLimit: (retryAfter, options) => {
      octokit.log.warn(
        `Request quota exhausted for request ${options.method} ${options.url}`
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
        `Abuse detected for request ${options.method} ${options.url}`
      );
    },
  },
});

async function getAllReleases() {
  return await octokit.paginate(octokit.rest.repos.listReleases, {
    owner: "open-goal",
    repo: "jak-project",
    per_page: 100,
  });
}

let requestedVersion = process.env.JAK_PROJ_VERSION;
let release = undefined;
if (requestedVersion === "latest") {
  const { data: releaseData } = await octokit.rest.repos.getLatestRelease({
    owner: "open-goal",
    repo: "jak-project",
  });
  release = releaseData;
} else {
  if (requestedVersion.startsWith("^")) {
    // Caret syntax, search for the maximum minor/patch without changing major
    let [major, minor, patch] = requestedVersion
      .replace("^", "")
      .split(".")
      .map((str) => parseInt(str));
    // Releases are not guaranteed to be in order, so we have to get them all and find the max
    let highestMinor = -1;
    let highestPatch = -1;
    let pickedRelease = undefined;
    let releases = await getAllReleases();
    for (const release of releases) {
      let [tagMajor, tagMinor, tagPatch] = release.tag_name
        .replace("v", "")
        .split(".")
        .map((str) => parseInt(str));
      if (tagMajor != major) {
        continue;
      }
      if (tagMinor > highestMinor) {
        highestMinor = tagMinor;
        highestPatch = tagPatch;
        pickedRelease = release;
      } else if (tagPatch > highestPatch && tagMinor == highestMinor) {
        highestPatch = tagPatch;
        pickedRelease = release;
      }
    }
    if (pickedRelease !== undefined) {
      release = pickedRelease;
    }
  } else if (requestedVersion.startsWith("v")) {
    // Given a specific tag, go fetch it
    const { data: releaseData } = await octokit.rest.repos.getReleaseByTag({
      owner: "open-goal",
      repo: "jak-project",
      tag: requestedVersion,
    });
    release = releaseData;
  }
}

if (release === undefined) {
  console.log(`Couldn't find a release with ${requestedVersion}`);
  exit(1);
}

console.log(`Using - ${release.tag_name}`);

let platform = process.env.PLATFORM;

// Get all assets
const { data: releaseAssets } = await octokit.rest.repos.listReleaseAssets({
  owner: "open-goal",
  repo: "jak-project",
  release_id: release.id,
  per_page: 100,
});

// Find the right one and download it to the right folder
let downloadDir = process.env.DOWNLOAD_DIR;
for (var i = 0; i < releaseAssets.length; i++) {
  const asset = releaseAssets[i];
  if (asset.name.toLowerCase().startsWith(`opengoal-${platform}`)) {
    const assetDownload = await octokit.rest.repos.getReleaseAsset({
      owner: "open-goal",
      repo: "jak-project",
      asset_id: asset.id,
      headers: {
        Accept: "application/octet-stream",
      },
    });
    await fs.promises.writeFile(
      path.join(downloadDir, asset.name),
      Buffer.from(assetDownload.data)
    );
  }
}

// Write out the version we grabbed
const binaryBundleMeta = {
  version: release.tag_name,
};
fs.writeFileSync(
  "./metadata.json",
  JSON.stringify(binaryBundleMeta, null, 2) + "\n"
);
