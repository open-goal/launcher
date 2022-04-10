import { series } from 'async';
import { extract } from './extract';
import { open } from '@tauri-apps/api/dialog';
import { platform } from '@tauri-apps/api/os';
import { appDir, join } from '@tauri-apps/api/path';
import { copyFile, createDir, readDir } from '@tauri-apps/api/fs';
import { Command } from '@tauri-apps/api/shell';
import { buildGame } from './launch';

const userDir = await appDir();
let jakISOPath;

// refactored tauri get iso function
// async function getISO(callback) {
//     const isoPath = await open({ multiple: false, directory: false, filters: [{ extensions: ['ISO'], name: 'Jak ISO File' }] });

//     if (isoPath) {
//         jakISOPath = isoPath;
//         return ('Got the ISO File');
//     }
//     throw new Error('No ISO File Selected');
// }

function getISO(callback) {
    async function helper() {
        const isoPath = await open({ multiple: false, directory: false, filters: [{ extensions: ['ISO'], name: 'Jak ISO File' }] });

        if (isoPath) {
            jakISOPath = isoPath;
            return [null, 'Got the ISO File'];
        }
        return ['No ISO File Selected', null];
    }

    helper()
        .then(res => {
            const [err, response] = res;
            if (err) return callback(err, null);
            if (response) return callback(null, response);
        })
}

// refactored tauri copy jak iso function
function copyJakISO(callback) {
    async function helper() {
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
        return [null, 'Copied ISO to proper directory!'];
    }

    helper()
        .then(res => {
            const [err, response] = res;
            if (err) return callback(err, null);
            if (response) return callback(null, response);
        })
}

function runDecompiler(callback) {
    async function helper() {
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
                // return ['Unsupported OS', null];
                break;
            case 'linux':
                decompCommand = 'decomp-linux';
                // return ['Unsupported OS', null];
                break;
            default:
                // return ['Unsupported OS', null];
                break;
        }

        const decomp = new Command(decompCommand, null, { cwd: jakDecompPath });
        decomp.on('close', data => {
            console.log(`Decomp finished with code ${data.code} and signal ${data.signal}`);
            return [null, 'Decomp finished close'];
        });
        decomp.on('error', error => {
            console.log(`Decomp error: "${error}"`);
        });
        decomp.stdout.on('data', line => {
            console.log(`Decomp stdout: "${line}"`)
            if (line.includes('Disassembly has completed successfully')) {
                return [null, 'Decomp finished successful'];
            }
        });

        const child = await decomp.execute();
    }

    helper()
        .then(res => {
            console.log(res);
            const [err, response] = res;
            if (err) return callback(err, null);
            if (response) return callback(null, response);
        });
}

export function isoSeries() {
    series([getISO, copyJakISO, extract, runDecompiler, buildGame], (err, result) => {
        if (err) return console.log(err);
        if (result) return console.log(result);
    });
}