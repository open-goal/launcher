import { writable } from 'svelte/store';

const InstallStore = writable([
  {
    isInstalling: false,
  }, {
    currentStatus: { status: "", percent: 0 }
  }
]);

export default InstallStore;