/**
 * Semantic Version
 * @pattern ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$
 */
type Semver = string;

type SupportedGame = "jak1" | "jak2" | "jak3" | "jakx";

interface ModVersion {
  version: string;
  publishedDate: string;
  assets: Record<string, string | null>;
  supportedGames?: SupportedGame[];
}

interface ModPerGameConfig {
  coverArtUrl?: string;
  thumbnailArtUrl?: string;
  releaseDate?: string;
}

interface ModInfo {
  displayName: string;
  description: string;
  authors: string[];
  tags: string[];
  supportedGames: SupportedGame[];
  websiteUrl?: string;
  versions: ModVersion[];
  perGameConfig?: Record<string, ModPerGameConfig>;
  coverArtUrl?: string;
  thumbnailArtUrl?: string;
  externalLink?: string;
}

export interface ModSourceData {
  schemaVersion: Semver;
  sourceName: string;
  lastUpdated: string;
  mods: Record<string, ModInfo>;
  texturePacks: Record<string, ModInfo>;
}
