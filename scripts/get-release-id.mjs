// GitHub's API for getting a release by tag ignores drafts sadly
// So we have to list and find it ourselves using the tag name...
//
// TODO - make this an action

import { Octokit } from "@octokit/rest";
import { throttling } from "@octokit/plugin-throttling";
import { retry } from "@octokit/plugin-retry";

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

// Get all values from workflow
const releaseTag = process.env.TAG_NAME;

if (releaseTag === undefined || releaseTag === "") {
  console.log("You didn't provide TAG_NAME");
  process.exit(1);
}

// Pull down the `launcher` release metadata
const releases = await octokit.rest.repos.listReleases({
  owner: owner,
  repo: repo,
  per_page: 100,
});

// NOTE - assumes the release is in the first page
for (const release of releases.data) {
  if (release.tag_name === releaseTag) {
    console.log(release.id);
    process.exit(0);
  }
}

console.log("Couldn't find release with that tag");
process.exit(1);
