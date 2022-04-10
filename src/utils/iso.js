import { extract } from './extract';
import { message, open } from '@tauri-apps/api/dialog';
import { platform } from '@tauri-apps/api/os';
import { appDir, join } from '@tauri-apps/api/path';
import { copyFile, createDir, readDir } from '@tauri-apps/api/fs';
import { Command } from '@tauri-apps/api/shell';
import { buildGame } from './launch';

const userDir = await appDir();
let jakISOPath;

async function getISO() {
    const isoPath = await open({ multiple: false, directory: false, filters: [{ extensions: ['ISO'], name: 'Jak ISO File' }] });

    if (isoPath) {
        jakISOPath = isoPath;
        return 'Got the ISO File';
    }
    throw new Error('No ISO File Selected');
}

async function copyJakISO() {
    const destinationPath = await join(userDir, '/iso/');

    try {
        const files = await readDir(destinationPath);
    } catch (error) {
        console.log(error.message);
        await createDir(destinationPath);
    }

    const destination = await join(userDir, '/iso/jak.iso');
    console.log("Copy Destination:", destination);
    const copyout = await copyFile(jakISOPath, destination);
    return 'Copied ISO to proper directory!';

}

async function runDecompiler() {
    const userPlatform = await platform();
    const jakprojectPath = await join(await userDir, '/jak-project');
    const jakDecompPath = await join(userDir, '/jak-project/scripts/batch/');
    let decompCommand;

    switch (userPlatform) {
        case 'win32':
            decompCommand = 'decomp-windows';
            break;
        case 'darwin':
            decompCommand = 'decomp-mac';
            throw new Error('Unsupported OS');
            break;
        case 'linux':
            decompCommand = 'decomp-linux';
            throw new Error('Unsupported OS');
            break;
        default:
            throw new Error('Unsupported OS');
            break;
    }

    const decomp = new Command(decompCommand, null, { cwd: jakDecompPath });
    decomp.on('close', data => {
        console.log(`Decomp finished with code ${data.code} and signal ${data.signal}`);
        return 'Decomp finished close';
    });
    decomp.on('error', error => {
        console.log(`Decomp error: "${error}"`);
    });
    decomp.stdout.on('data', line => {
        console.log(`Decomp stdout: "${line}"`)
        if (line.includes('Disassembly has completed successfully')) {
            return 'Decomp finished successful';
        }
    });

    const child = await decomp.execute();
}

export async function isoSeries() {
    return await Promise.resolve()
        .then(getISO)
        .then(copyJakISO)
        .then(extract)
        .then(runDecompiler)
        .then(buildGame)
        .then(res => message(res))
        .catch(err => {
            console.log(err);
            message(err.message);
        });
}