interface VersionInfo {
  installed: boolean;
  installedVersion: string | undefined;
}

interface VersionState {
  activeToolingVersion: string | undefined;
  activeModVersionInfo: VersionInfo;
  displayModVersion: boolean;
}

export const versionState: VersionState = $state({
  activeToolingVersion: undefined,
  activeModVersionInfo: {
    installed: true,
    installedVersion: undefined,
  },
  displayModVersion: false,
});
