import { appDir, join } from '@tauri-apps/api/path';
import { platform } from '@tauri-apps/api/os';

// const _7z = require('7zip')['7z'];

// extract game files from iso -> into jak-project/iso_data/jak1 dir
export async function extract(callback) {
  if (platform() == 'win32') {
    const workingDirPath = await appDir();
    const isoFilePath = await join(workingDirPath, '/iso/jak.iso');
    const outputDirPath = await join(workingDirPath, '/jak-project/iso_data/jak1/');
    console.log("Output Directory:", outputDirPath);

    return callback(null, 'extracted iso');

    // x: extract with full paths
    // -y: yes to all prompts
    // -w: working directory (appdata/opengoal-launcher dir)
    // -o: output directory (appdata/opengoal-launcher dir/jak-project/iso_data/jak1/)


    // const task = spawn(_7z, ['x', isoFilePath, '-y', `-w${workingDirPath}`, `-o${outputDirPath}`]);


    // task.stdout.on('data', (data) => {
    //   console.log('stdout: ', data.toString('utf8'));
    // });
    // task.stderr.on('data', (data) => {
    //   console.log('stderr: ', data.toString('utf8'));
    // });
    // task.on('close', (code, signal) => {
    //   console.log('Exited with code: ', code);
    //   if (code === 0) {
    //     callback(null, 'ISO assests extracted successfully');
    //     return;
    //   } else {
    //     callback(signal, null);
    //     return;
    //   }
    // });
  } else {
    return callback('Unsupported OS');
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