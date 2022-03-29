const { send, receive, handleStatus, handleConsole } = window.electronAPI;

function checkUpdates() {
  send('checkUpdates');
}