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

let tagToSearchFor = process.env.TAG_VALUE.split("refs/tags/")[1];

const { data: recentReleases } = await octokit.rest.repos.listReleases({
  owner: "open-goal",
  repo: "launcher",
  per_page: 100,
});

let release = undefined;
for (var i = 0; i < recentReleases.length; i++) {
  if (recentReleases[i].tag_name == tagToSearchFor) {
    release = recentReleases[i];
    break;
  }
}

if (release === undefined) {
  console.log(`Could not find release with tag name: ${tagToSearchFor}`);
  process.exit(1);
}

// TODO - no macOS yet
const releaseMeta = {
  name: release.tag_name,
  notes: "UPDATE",
  pub_date: release.created_at,
  platforms: {
    linux: {
      signature: "",
      url: `https://github.com/open-goal/launcher/releases/download/${
        release.tag_name
      }/opengoal-launcher_${tagToSearchFor.replace(
        "v",
        ""
      )}_amd64.AppImage.tar.gz`,
    },
    win64: {
      signature: "",
      url: `https://github.com/open-goal/launcher/releases/download/${
        release.tag_name
      }/opengoal-launcher_${tagToSearchFor.replace("v", "")}_x64_en-US.msi.zip`,
    },
  },
};
fs.writeFileSync(
  "./.tauri/latest-release.json",
  JSON.stringify(releaseMeta, null, 2)
);

await octokit.rest.repos.updateRelease({
  owner: "open-goal",
  repo: "launcher",
  release_id: release.id,
  draft: false,
});
