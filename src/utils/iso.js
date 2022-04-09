import { extract } from './extract';
// import { buildGame } from './launch';
import { open } from '@tauri-apps/api/dialog';
import { platform } from '@tauri-apps/api/os';
import { appDir, join } from '@tauri-apps/api/path';
import { copyFile, createDir, readDir } from '@tauri-apps/api/fs';
import { series } from 'async';

const userDir = await appDir();
const userPlatform = await platform();
let isoPath;

// finished tauri get iso function
async function getISO(callback) {
    const iso = await open({ options: { multiple: false, directory: false, filters: { extensions: ["iso", "ISO"] } } });
    if (iso) {
        isoPath = iso;
        callback(null, 'Received the ISO File from the user!');
    }
}

// finished tauri copy jak iso function
async function copyJakISO(callback) {
    const destinationPath = await join(userDir, '/iso/');

    try {
        const files = await readDir(destinationPath);
    } catch (error) {
        console.log(error);
        await createDir(destinationPath);
    }

    const destination = await join(userDir, '/iso/jak.iso');
    console.log(destination);
    const copyout = await copyFile(isoPath, destination);
    callback(null, 'Copied Jak ISO to correct directory');
}

async function runDecompiler(callback) {
    switch (userPlatform) {
        case 'win32':
            console.log('windows');
            break;
        case 'darwin':
            console.log('mac');
            callback('Unsupported OS', null);
            break;
        case 'linux':
            console.log('linux');
            break;
        default:
            break;
    }
}

// function runDecompiler(callback) {
//         let decomp = exec(decompScript, { shell: true, cwd: path.join(jakprojectPath, '/scripts/batch/') });

//         decomp.stdout.on('data', data => {
//             console.log(data);
//         });

//         // i see people online saying to use decomp.on('exit'), but thats triggering this listener immediately upon running
//         decomp.on('close', (code => {
//             if (code === 0) {
//                 callback(null, 'ISO decompiled successfully');
//                 return;
//         }));
//     }
// }

export function isoSeries() {
    // series([getISO, copyJakISO, extract, runDecompiler, buildGame], (err, result) => {
    series([getISO, copyJakISO, runDecompiler, extract], (err, result) => {
        if (err) console.log(err);
        if (result) console.log(result);
    });
}