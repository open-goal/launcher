interface SystemInfoState {
  isMinVCCRuntimeInstalled: boolean;
}

export const systemInfoState: SystemInfoState = $state({
  isMinVCCRuntimeInstalled: false,
});
