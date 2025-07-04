// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ModPerGameConfig } from "./ModPerGameConfig";
import type { ModVersion } from "./ModVersion";
import type { SupportedGame } from "./SupportedGame";

export type ModInfo = {
  displayName: string;
  description: string;
  authors: Array<string>;
  tags: Array<string>;
  supportedGames: Array<SupportedGame>;
  websiteUrl: string | null;
  versions: Array<ModVersion>;
  perGameConfig: { [key in string]?: ModPerGameConfig } | null;
  coverArtUrl: string | null;
  thumbnailArtUrl: string | null;
  externalLink: string | null;
  name?: string;
  source?: string;
};
