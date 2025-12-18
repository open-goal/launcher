export interface PackInfo {
  name: string;
  enabled: boolean;
  toBeDeleted: boolean;
}

export interface PackMetadata {
  fileList: string[];
  coverImagePath: string;
  name: string;
  version: string;
  author: string;
  releaseDate: string;
  description: string;
  tags: string[];
}
