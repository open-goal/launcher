const { spawn } = require('child_process');

const libPath = '../../assets/7zip/';

function extract() {
  const child = spawn('execute', ['7z2107-arm64.exe', libPath, '-packxz', filePath]);

  child.stdout.on('data', (data) => {
    console.log('stdout: ', data.toString('utf8'));
  });
  child.stderr.on('data', (data) => {
    console.log('stderr: ', data.toString('utf8'));
  });
  child.on('close', (code, signal) => {
    console.log('Exited with code: ', code);
    resolve();
  });
}


module.exports = {
  extract
}