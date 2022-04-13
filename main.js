import './style.css'
import { actions, general_pane, files_pane, links_pane } from './components/settings/settings';
import { launchGame } from './src/utils/launch';
import { initConfig, getInstallStatus, getLastActiveGame, SupportedGame, setInstallStatus } from './src/config/config';
import { mainTemplate, setupTemplate } from './src/views/main.dot';
import { sidebarTemplate } from './src/views/sidebar.dot';
import { filePrompt } from './src/utils/file'
import { compileGame, decompileGameData, extractISO, validateGameData } from './src/utils/setup';

// Main App Startup Routine
(async function () {
  // Create the default or load the config
  await initConfig();
  // Render the App
  await renderApp();
}());

// State
let settingUpGame = false;
let activeGame;

// Main App Rendering Handler
async function renderApp() {
  activeGame = await getLastActiveGame();
  const gameInstalled = await getInstallStatus(activeGame);
  document.getElementById("sidebar").innerHTML = sidebarTemplate({
    jak1Active: activeGame.name == SupportedGame.Jak1.name,
    jak2Active: activeGame.name == SupportedGame.Jak2.name,
    jak3Active: activeGame.name == SupportedGame.Jak3.name,
    jakxActive: activeGame.name == SupportedGame.JakX.name,
  });

  if (!settingUpGame) {
    document.getElementById("main").innerHTML = mainTemplate({
      gameInstalled: gameInstalled
    });

    // Setup Event Handlers
    if (gameInstalled) {
      document.getElementById("playBtn").onclick = () => {
        launchGame();
      }
      document.getElementById("configBtn").onclick = () => {
        // TODO
      }
    } else {
      document.getElementById("setupBtn").onclick = async () => {
        settingUpGame = true;
        renderSetupProcess();
      }
    }
  } else {
    await renderSetupProcess();
  }

  const sidebar = document.getElementById('sidebar');

  // TODO - not cleaned up yet!
  sidebar.onclick = e => {
    switch (e.target.getAttribute('key')) {
      case 'return':
        document.getElementById('main').innerHTML = jak1_main;
        document.getElementById('sidebar').innerHTML = jak1_sidebar;
        break;
      case 'settings':
        document.getElementById('main').innerHTML = general_pane;
        document.getElementById('sidebar').innerHTML = actions;
        break;
      case 'general':
        document.getElementById('main').innerHTML = general_pane;
        break;
      case 'links':
        document.getElementById('main').innerHTML = links_pane;
        break;
      case 'files':
        document.getElementById('main').innerHTML = files_pane;
        break;
    }
  }
}

async function renderSetupProcess() {
  document.getElementById("main").innerHTML = setupTemplate({});

  // Setup Event Handlers
  document.getElementById("browseForIsoBtn").onclick = async () => {
    // Get the file path
    let isoPath = await filePrompt();
    document.getElementById("filePathLabel").innerHTML = isoPath;
    // Disable the cancel button, the user has committed!
    document.getElementById("cancelBtn").disabled = true;
    // Extract ISO
    document.getElementById("progressExtract").innerHTML = `<div class="loader"></div>Extracting ISO`;
    let output = await extractISO(isoPath);
    document.getElementById("progressExtract").innerHTML = `${output.code === 0 ? "✅" : "❌"} Extracting ISO`;
    // TODO - logs should go to a file
    document.getElementById("installLogs").innerHTML += output.stdout.replaceAll("\n", "<br>") + output.stderr.replaceAll("\n", "<br>");
    // Validate
    if (output.code === 0) {
      document.getElementById("progressValidating").innerHTML = `<div class="loader"></div>Validating Game Data`;
      output = await validateGameData(isoPath);
      document.getElementById("progressValidating").innerHTML = `${output.code === 0 ? "✅" : "❌"} Validating Game Data`;
      document.getElementById("installLogs").innerHTML += output.stdout.replaceAll("\n", "<br>") + output.stderr.replaceAll("\n", "<br>");
    }
    // Decompiling
    if (output.code === 0) {
      document.getElementById("progressDecompile").innerHTML = `<div class="loader"></div>Decompiling Game Data`;
      output = await decompileGameData(isoPath);
      document.getElementById("progressDecompile").innerHTML = `${output.code === 0 ? "✅" : "❌"} Decompiling Game Data`;
      document.getElementById("installLogs").innerHTML += output.stdout.replaceAll("\n", "<br>") + output.stderr.replaceAll("\n", "<br>");
    }
    // Compile
    if (output.code === 0) {
      document.getElementById("progressCompile").innerHTML = `<div class="loader"></div>Compiling the Game`;
      output = await compileGame(isoPath);
      document.getElementById("progressCompile").innerHTML = `${output.code === 0 ? "✅" : "❌"} Compiling the Game`;
      document.getElementById("installLogs").innerHTML += output.stdout.replaceAll("\n", "<br>") + output.stderr.replaceAll("\n", "<br>");
    }
    // Update Settings
    if (output.code === 0) {
      await setInstallStatus(activeGame, true);
      settingUpGame = false;
      // Re-Render
      renderApp();
    }
  }
  document.getElementById("cancelBtn").onclick = () => {
    settingUpGame = false;
    renderApp();
  }
}
