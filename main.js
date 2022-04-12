import './style.css'
import { actions, general_pane, files_pane, links_pane } from './components/settings/settings';
import { isoSeries } from './src/utils/iso';
import { launchGame } from './src/utils/launch';
import { initConfig, getInstallStatus, getLastActiveGame, SupportedGame } from './src/config/config';
import { mainTemplate } from './src/views/main.dot';
import { sidebarTemplate } from './src/views/sidebar.dot';

// Main App Startup Routine
(async function () {
  // Create the default or load the config
  await initConfig();
  // Render the App
  await renderApp();
}());

// Main App Rendering Handler
async function renderApp() {
  const activeGame = await getLastActiveGame();
  const gameInstalled = await getInstallStatus(activeGame);
  document.getElementById("sidebar").innerHTML = sidebarTemplate({
    jak1Active: activeGame.name == SupportedGame.Jak1.name,
    jak2Active: activeGame.name == SupportedGame.Jak2.name,
    jak3Active: activeGame.name == SupportedGame.Jak3.name,
    jakxActive: activeGame.name == SupportedGame.JakX.name,
  });
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
    document.getElementById("setupBtn").onclick = () => {
      isoSeries();
    }
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
