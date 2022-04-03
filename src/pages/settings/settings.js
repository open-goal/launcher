const { send, receive, handleStatus, handleConsole } = window.electronAPI;

function handleClick(target) {
  const key = target.getAttribute('key');
  const panes = ['general', 'files', 'links'];

  if (key) {
    let keyPane = document.querySelector(`#${key}`);

    panes.filter(pane => pane !== key).map(pane => {
      let div = document.querySelector(`#${pane}`);
      if (div) {
        div.style.display = 'none';
      }
    });

    if (keyPane) {
      keyPane.style.display = 'flex';
    }
  }
}