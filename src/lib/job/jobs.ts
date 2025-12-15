const JOB_TYPES = [
  "installGame",
  "decompile",
  "compile",
  "updateGame",
  "applyTexturePacks",
  "installModFromUrl",
  "installModLocally",
  "decompileMod",
  "compileMod",
] as const;

export type JobType = typeof JOB_TYPES[number];

export function asJobType(value: JobType): JobType {
  return value;
}

export function isJobType(value: string): value is JobType {
  return (JOB_TYPES as readonly string[]).includes(value);
}

export function toJobType(value?: string): JobType | undefined {
  if (!value) {
    return undefined;
  }
  return isJobType(value) ? value : undefined;
}
