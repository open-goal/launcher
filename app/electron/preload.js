const { contextBridge, ipcRenderer } = require("electron");
const fs = require("fs");
const i18nextBackend = require("i18next-electron-fs-backend");
const Store = require("secure-electron-store").default;
const ContextMenu = require("secure-electron-context-menu").default;
const SecureElectronLicenseKeys = require("secure-electron-license-keys");

// Create the electron store to be made available in the renderer process
const store = new Store();

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld("api", {
  i18nextElectronBackend: i18nextBackend.preloadBindings(ipcRenderer, process),
  store: store.preloadBindings(ipcRenderer, fs),
  contextMenu: ContextMenu.preloadBindings(ipcRenderer),
  licenseKeys: SecureElectronLicenseKeys.preloadBindings(ipcRenderer),
  // were gonna figure out redux store in the future, but for now this is how im gonna communicate between front-end/backend
  send: (command) => {
    let validCommands = ['getISO', 'checkUpdates', 'launch', 'build'];
    if (validCommands.includes(command)) {
      ipcRenderer.send(command);
    }
  },
  receive: (channel, func) => {
    let validChannels = ["fromMain", 'status'];
    if (validChannels.includes(channel)) {
      // Deliberately strip event as it includes `sender` 
      ipcRenderer.on(channel, (event, ...args) => func(...args));
    }
  }
});
