import './style.css'
import { actions, general_pane, files_pane, links_pane } from './components/settings/settings';
import { jak1_main, jak1_sidebar } from './components/jak1/jak1';
import { isoSeries } from './src/utils/iso';
import { launchGame } from './src/utils/launch';

const sidebar = document.querySelector('.sidebar');

sidebar.onclick = e => {
  switch (e.target.getAttribute('key')) {
    case 'return':
      document.querySelector('.main').innerHTML = jak1_main;
      document.querySelector('.sidebar').innerHTML = jak1_sidebar;
      break;
    case 'settings':
      document.querySelector('.main').innerHTML = general_pane;
      document.querySelector('.sidebar').innerHTML = actions;
      break;
    case 'general':
      document.querySelector('.main').innerHTML = general_pane;
      break;
    case 'links':
      document.querySelector('.main').innerHTML = links_pane;
      break;
    case 'files':
      document.querySelector('.main').innerHTML = files_pane;
      break;
  }
  renderControls();
}

class SupportedGame {
  static Jak1 = new SupportedGame("Jak 1")
  static Jak2 = new SupportedGame("Jak 2")
  static Jak3 = new SupportedGame("Jak 3")
  static JakX = new SupportedGame("Jak X")

  constructor(name) {
    this.name = name
  }
}

function isGameSetup(supportedGame) {
  if (supportedGame == SupportedGame.Jak1) {
    // TODO - check the installation directory and such, wherever that ends up being
    // return false for now
    return false;
  } else {
    return false;
  }
}

// TODO - detect last played game, right now its assumed that jak 1 is always selected by default
let activeGame = SupportedGame.Jak1;

renderControls();

function renderControls() {
  if (document.getElementById("launcherControls") == undefined) {
    return;
  }
  if (isGameSetup(activeGame)) {
    document.getElementById("launcherControls").innerHTML += `<button id="configBtn">CONFIG</button><button id="playBtn">PLAY</button>`;
    document.getElementById("playBtn").onclick = () => {
      launchGame();
    }
    document.getElementById("configBtn").onclick = () => {
      // TODO
    }
  } else {
    document.getElementById("launcherControls").innerHTML += `<button id="setupBtn">SETUP</button>`;
    document.getElementById("setupBtn").onclick = () => {
      isoSeries();
    }
  }
}
