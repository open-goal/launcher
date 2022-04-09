import { waterfall } from 'async';
import { extract } from './extract';
// import { buildGame } from './launch';
import { open } from '@tauri-apps/api/dialog';
import { platform } from '@tauri-apps/api/os';
import { appDir, join } from '@tauri-apps/api/path';
import { copyFile, createDir, readDir } from '@tauri-apps/api/fs';
import { Command } from '@tauri-apps/api/shell';

const userDir = await appDir();
const userPlatform = await platform();

// refactored tauri get iso function
async function getISO(callback) {
    const isoPath = await open({ options: { multiple: false, directory: false, filters: { extensions: ["iso", "ISO"] } } });
    if (isoPath) {
        return (null, isoPath);
    }
}

// refactored tauri copy jak iso function
async function copyJakISO(isoPath, callback) {
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
    return (null, 'Copied ISO to proper directory!');
}

async function runDecompiler(callback) {
    switch (userPlatform) {
        case 'win32':
            console.log('windows');
            return ('sad');
            break;
        case 'darwin':
            console.log('mac');
            return ('Unsupported OS');
            break;
        case 'linux':
            console.log('linux');
            return ('Unsupported OS');
            break;
        default:
            return ('Unsupported OS');
            break;
    }

    // new Command('')
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

export function isoWaterfall() {
    waterfall([
        getISO,
        copyJakISO,
        extract,
        runDecompiler
    ], function (err, result) {
        if (err) console.log(err);
        if (result) console.log(result);
    });
}