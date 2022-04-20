import { Octokit } from "@octokit/rest";
import { throttling } from "@octokit/plugin-throttling";
import { retry } from "@octokit/plugin-retry";
import * as fs from 'fs';
import * as path from 'path';

Octokit.plugin(throttling);
Octokit.plugin(retry);
const octokit = new Octokit({
  auth: process.env.GITHUB_TOKEN,
  userAgent: 'open-goal/jak-project',
  log: {
    debug: () => { },
    info: () => { },
    warn: console.warn,
    error: console.error
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
  }
});

let requestedVersion = process.env.JAK_PROJ_VERSION;
let release = undefined;
if (requestedVersion === "latest") {
  const { data: releaseData } = await octokit.rest.repos.getLatestRelease({
    owner: "open-goal",
    repo: "jak-project"
  });
  release = releaseData;
} else {
  const { data: releaseData } = await octokit.rest.repos.getReleaseByTag({
    owner: "open-goal",
    repo: "jak-project",
    tag: requestedVersion
  });
  release = releaseData;
}

let platform = process.env.PLATFORM;
if (platform.includes("windows")) {
  platform = "windows";
} else if (platform.includes("linux")) {
  platform = "linux";
} else {
  platform = "macos";
}

// Get all assets
const { data: releaseAssets } = await octokit.rest.repos.listReleaseAssets({
  owner: "open-goal",
  repo: "jak-project",
  release_id: release.id,
  per_page: 100
});

// Find the right one and download it to the right folder
let downloadDir = process.env.DOWNLOAD_DIR;
for (var i = 0; i < releaseAssets.length; i++) {
  const asset = releaseAssets[i];
  if (asset.name.toLowerCase().includes(platform)) {
    const assetDownload = await octokit.rest.repos.getReleaseAsset({
      owner: "open-goal",
      repo: "jak-project",
      asset_id: asset.id,
      headers: {
        "Accept": "application/octet-stream"
      }
    });
    await fs.promises.writeFile(path.join(downloadDir, asset.name), Buffer.from(assetDownload.data));
  }
}
