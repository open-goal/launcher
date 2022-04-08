const { app } = require('electron');
const path = require('path');
const { spawn } = require('child_process');
const _7z = require('7zip')['7z'];

function updateStatus(status) {
  app.emit('status', status);
}

// extract game files from iso -> into jak-project/iso_data/jak1 dir
function extract(callback) {
  if (process.platform == 'win32') {
    updateStatus('Extracting assets from ISO File');
    // x: extract with full paths
    // -y: yes to all prompts
    // -w: working directory (appdata/opengoal-launcher dir)
    // -o: output directory (appdata/opengoal-launcher dir/jak-project/iso_data/jak1/)
    const workingDirPath = app.getPath('userData');
    const isoFilePath = `${workingDirPath}/iso/jak.iso`;
    const outputDirPath = path.join(workingDirPath, '/jak-project/iso_data/jak1/');

    const task = spawn(_7z, ['x', isoFilePath, '-y', `-w${workingDirPath}`, `-o${outputDirPath}`]);
    task.stdout.on('data', (data) => {
      console.log('stdout: ', data.toString('utf8'));
    });
    task.stderr.on('data', (data) => {
      console.log('stderr: ', data.toString('utf8'));
    });
    task.on('close', (code, signal) => {
      console.log('Exited with code: ', code);
      if (code === 0) {
        updateStatus('ISO assests extracted successfully');
        callback(null, 'ISO assests extracted successfully');
        return;
      } else {
        updateStatus(signal);
        callback(signal, null);
        return;
      }
    });
  } else {
    updateStatus('Unsupported OS');
    callback('Unsupported OS', null);
  }
}

// function macISOExtract() {
//   if (process.platform === 'darwin') {
//     const isoFile = path.join(isoSavePath, '/jak.iso');
//     const output = path.join(isoSavePath, '/jak1');
//     const archive = new ArchiveHdi(isoFile);
//     await archive.read(async entry => {
//       console.log(entry.path);
//       await entry.extract(`${output}/${entry.path}`);
//     });
//   }
// }

module.exports = {
  extract
}