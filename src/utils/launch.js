import { message } from "@tauri-apps/api/dialog";
import { platform } from "@tauri-apps/api/os";
import { appDir, join } from "@tauri-apps/api/path";
import { Command } from "@tauri-apps/api/shell";
import { setInstallStatus } from '../config/config';

export async function buildGame() {
    const userPlatform = await platform();
    const jakLaunchPath = await join(await appDir(), '/jak-project/out/build/Release/bin/');

    let compilerScript = null;

    if (userPlatform === 'win32') {
        compilerScript = 'compile-windows';
    } else if (userPlatform === 'linux') {
        compilerScript = 'compile-linux';
    } else if (userPlatform === 'darwin') {
        compilerScript = 'compile-mac';
    }

    if (compilerScript) {
        const compile = new Command(compilerScript, null, { cwd: jakLaunchPath });
        compile.on('close', data => {
            console.log(`Compiler finished with code ${data.code} and signal ${data.signal}`);
            if (data.code === 0) {
                message('Game Ready to play!');
                setInstallStatus(SupportedGame.Jak1, true);
                return 'Compiler finished';
            } else {
                message('Game Ready to play!');
                throw new Error('Compiler finished');
            }

        });
        compile.on('error', error => {
            console.error(`Compiler error: "${error}"`);
        });
        compile.stdout.on('data', line => console.log(`Compiler stdout: "${line}"`));

        const child = await compile.spawn();
    }
}

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
            return 'Launch finished';
        });
        launch.on('error', error => {
            console.error(`Launch error: "${error}"`);
        });
        launch.stdout.on('data', line => console.log(`Launch stdout: "${line}"`));

        const child = await launch.spawn();
    }
}
