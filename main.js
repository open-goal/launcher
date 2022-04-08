import './style.css'
import { actions, general_pane, files_pane, links_pane } from './components/settings/settings';
import { jak1_main, jak1_sidebar } from './components/jak1/jak1';
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog';

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
}

const playBTN = document.querySelector('.play');
const configBTN = document.querySelector('.config');

playBTN.onclick = () => {
  invoke('launch')
}

configBTN.onclick = async () => {
  const iso = await open({ options: { multiple: false, directory: true, filters: { extensions: ['iso'] } } });
  console.log(iso);
}