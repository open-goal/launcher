interface SystemInfoState {
  isMinVCCRuntimeInstalled: boolean;
  isMinMacOSVersion: boolean | undefined;
}

export const systemInfoState: SystemInfoState = $state({
  isMinVCCRuntimeInstalled: false,
  isMinMacOSVersion: undefined,
});
