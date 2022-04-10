import { appDir, join } from '@tauri-apps/api/path';
import { Command } from '@tauri-apps/api/shell';
import { platform } from '@tauri-apps/api/os';

// extract game files from iso -> into jak-project/iso_data/jak1 dir
export async function extract() {
  const userDir = await appDir();
  const workingDir = await join(userDir, '/');
  const outputDir = await join(userDir, '/jak-project/iso_data/jak1/');
  const isoDir = await join(userDir, '/iso/jak.iso');

  console.log(workingDir);
  console.log(outputDir);
  console.log(isoDir);

  const extract = new Command('extract-windows', ['x', isoDir, 'y', `-w${userDir}`, `-o${outputDir}`]);
  extract.on('close', data => {
    console.log(`Extraction finished with code ${data.code} and signal ${data.signal}`);
    if (data.code === 0) {
      return 'Extraction finished';
    }
    throw new Error('Extraction exited with code:', data.code);
  });
  extract.on('error', error => {
    console.log(`Decomp error: "${error}"`);
    throw new Error('Extraction failed!');
  });
  extract.stdout.on('data', line => {
    console.log(`Extract stdout: "${line}"`)
  });

  const child = await extract.spawn();
}

  //  x: extract with full paths
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