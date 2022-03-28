const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');
const isDev = !app.isPackaged;
const { launchGame, buildGame, isoSeries, fetchLatestCommit } = require('./js/utils/utils.js');
let mainWindow;

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require('electron-squirrel-startup')) {
  // eslint-disable-line global-require
  app.quit();
}

const createWindow = () => {
  // Create the browser window.
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    resizable: false,
    title: "OpenGOAL Launcher",
    icon: path.join(__dirname, 'assets/images/icon.png'),
    webPreferences: {
      preload: path.join(__dirname, "/js/preload.js"),
      nodeIntegration: true,
      devTools: isDev
    }
  });

  // and load the index.html of the app.
  mainWindow.loadFile(path.join(__dirname, '/html/index.html'));

  // Open the DevTools.
  mainWindow.webContents.openDevTools();

  // hide menubar
  mainWindow.setMenuBarVisibility(false);
};

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on('ready', createWindow);

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  // On OS X it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and import them here.

// IPC STUFF HERE
ipcMain.on('getISO', isoSeries);

ipcMain.handle('checkUpdates', async () => {
  let response = await fetchLatestCommit();
  return response;
});

ipcMain.on('build', buildGame);
ipcMain.on('launch', launchGame);

// handle config status updates
app.on('status', (value) => {
  mainWindow.webContents.send('status', value);
});

// handle console messages
app.on('console', (value) => {
  mainWindow.webContents.send('console', value);
});
