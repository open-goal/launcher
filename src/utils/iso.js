import { series } from 'async';
import { extract } from './extract';
// import { buildGame } from './launch';
import { open } from '@tauri-apps/api/dialog';
import { platform } from '@tauri-apps/api/os';
import { appDir, join } from '@tauri-apps/api/path';
import { copyFile, createDir, readDir } from '@tauri-apps/api/fs';
import { message } from '@tauri-apps/api/dialog';
import { Command } from '@tauri-apps/api/shell';

const userDir = await appDir();
const userPlatform = await platform();
let jakISOPath;

// refactored tauri get iso function
async function getISO(callback) {
    const isoPath = await open({ multiple: false, directory: false, filters: [{ extensions: ['ISO'], name: 'Jak ISO File' }] });

    if (isoPath) {
        jakISOPath = isoPath;
        return ('Got the ISO File');
    }
    throw new Error('No ISO File Selected');
}

// refactored tauri copy jak iso function
async function copyJakISO(callback) {
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
    return ('Copied ISO to proper directory!');
}

async function runDecompiler(callback) {
    const jakprojectPath = await join(await userDir, '/jak-project');
    const jakDecompPath = await join(userDir, '/jak-project/scripts/batch/');
    let decompCommand;

    switch (userPlatform) {
        case 'win32':
            decompCommand = 'decomp-windows';
            break;
        case 'darwin':
            decompCommand = 'decomp-mac';
            return 'Unsupported OS';
            break;
        case 'linux':
            decompCommand = 'decomp-linux';
            return 'Unsupported OS';
            break;
        default:
            return 'Unsupported OS';
            break;
    }

    const decomp = new Command(decompCommand, null, { cwd: jakDecompPath });
    decomp.on('close', data => {
        console.log(`Decomp finished with code ${data.code} and signal ${data.signal}`);
        return ('Decomp finished');
    });
    decomp.on('error', error => {
        console.error(`Decomp error: "${error}"`);
    });
    decomp.stdout.on('data', line => console.log(`Decomp stdout: "${line}"`));

    const child = await decomp.spawn();
}

export async function isoSeries() {
    series([getISO, copyJakISO, extract, runDecompiler], (err, result) => {
        if (err) console.log(err);
        if (result) {
            console.log(result);
        }
    });
}