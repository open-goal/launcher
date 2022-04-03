const { contextBridge, ipcRenderer } = require("electron");

// Expose protected methods that allow the renderer process to use
contextBridge.exposeInMainWorld("electronAPI", {
  send: (command) => {
    let validCommands = ['getISO', 'checkUpdates', 'launch', 'build', 'settings', 'config', 'toggle-console'];
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
  },
  handleStatus: (callback) => ipcRenderer.on('status', callback),
  handleConsole: (callback) => ipcRenderer.on('console', callback)
});
