// const userDataPath = app.getPath('userData');
// const jakprojectPath = path.join(userDataPath, '/jak-project');

import { platform } from "@tauri-apps/api/os";
import { appDir, join } from "@tauri-apps/api/path";
import { Command } from "@tauri-apps/api/shell";

// export function buildGame(callback) {
//     let compilerScript = null;

//     if (process.platform === 'win32') {
//         compilerScript = path.join(jakprojectPath, '/out/build/Release/bin/goalc.exe');
//     } else if (process.platform === 'linux') {
//         compilerScript = path.join(jakprojectPath, '/scripts/shell/gc.sh');
//     } else if (process.platform === 'darwin') {
//         console.log('No Mac support at this time');
//         return;
//     }

//     if (compilerScript) {
//         updateStatus('Building the game');
//         // so its not console logging the '100%' when i run it in the series, but when i run it on its own its fine.
//         // so im going to assume its working properly and its a problem with the way the compiler is outputting the %%%
//         // for now i have a timeout that will kill the compiler process after 30 seconds because the compiler should be done by then (twice the length it takes my pc at least)
//         let build = execFile(compilerScript, ['-v', '-auto-user'], { timeout: 30000 });
//         build.stdout.on('data', data => {
//             console.log(data.toString().trim());
//             app.emit('console', data);
//             if (data.includes('[100%]')) {
//                 updateStatus('Compiled game successfully!');
//                 callback(null, 'Compiled game successfully!');
//                 return;
//             }
//         });

//         build.on('close', () => {
//             updateStatus('Compiled game successfully!');
//             callback(null, 'Compiled game successfully!');
//             return;
//         });

//         let stdinStream = new stream.Readable();
//         stdinStream.push('(mi)');
//         stdinStream.push(null);
//         stdinStream.pipe(build.stdin);
//     }
// }

export async function launchGame() {
    const userPlatform = await platform();
    const jaklaunchPath = await join(await appDir(), '/jak-project/scripts/batch/');
    let launchScript = null;

    if (userPlatform === 'win32') {
        launchScript = 'launch-windows';
    } else if (userPlatform === 'linux') {
        launchScript = 'launch-linux';
    } else if (userPlatform === 'darwin') {
        launchScript = 'launch-mac';
    }

    console.log(launchScript);

    if (launchScript) {
        const launch = new Command(launchScript, null, { cwd: jaklaunchPath });
        launch.on('close', data => {
            console.log(`Launch finished with code ${data.code} and signal ${data.signal}`);
            return ('Launch finished');
        });
        launch.on('error', error => {
            console.error(`Launch error: "${error}"`);
        });
        launch.stdout.on('data', line => console.log(`Launch stdout: "${line}"`));

        const child = await launch.spawn();
    }
}