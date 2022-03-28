const { dialog, app, shell } = require('electron');
const fs = require('fs-extra');
const path = require('path');
const { series } = require('async');
const { exec } = require('child_process');
const { buildGame } = require('./launch');

const userDataPath = app.getPath('userData');
const isoSavePath = path.join(userDataPath, '/iso');
const isoContents = path.join(userDataPath, '/iso/jak1')
const jakprojectPath = path.join(userDataPath, '/jak-project');
const isoFolderPath = path.join(jakprojectPath, '/iso_data');
// const jak1FolderPath = path.join(jakprojectPath, '/iso_data/jak1');

function updateStatus(status) {
    app.emit('status', status);
}

// get the ISO file from the user and save it
function getISOFile(callback) {
    const files = dialog.showOpenDialogSync(this.mainWindow, {
        message: 'Please select the Jak and Daxter ISO file',
        properties: ['openFile'],
        filters: [
            { name: 'ISO', extensions: ['iso', 'ISO', 'txt'] }
        ]
    });
    // if the user doesnt select an ISO file return
    if (!files) {
        updateStatus('Awaiting Jak ISO File');
        callback('Awaiting Jak ISO File', null);
        return;
    }

    // copying the iso file to the userdata/opengoal-launcher dir
    const isoPath = files[0];

    if (!fs.existsSync(isoSavePath)) {
        fs.mkdirSync(path.join(userDataPath, '/iso'));
    }
    fs.copyFileSync(isoPath, `${isoSavePath}/jak.iso`);
    updateStatus('Received Jak ISO File');
    callback(null, 'Received Jak ISO File');
}

// extract the game files from the main iso and put them into the jak1 folder
function extractISOFile(callback) {
    // if jak1 folder doesnt exist, then create the folder
    // ideally they can put the jak1 folder into their actual repo and this entire chunk can be deleted
    updateStatus('Extracting assets from ISO File');
    if (!fs.existsSync(path.join(isoSavePath, '/jak1'))) {
        fs.mkdirSync(path.join(isoSavePath, '/jak1'));
    }

    // if (!fs.existsSync(jak1FolderPath)) {
    //     fs.mkdirSync(path.join(jakprojectPath, '/iso_data/jak1'));
    // }

    // if (process.platform === 'darwin') {
    //     // const isoFile = path.join(isoSavePath, '/jak.iso');
    //     // const output = path.join(isoSavePath, '/jak1');
    //     // const archive = new ArchiveHdi(isoFile);
    //     // await archive.read(async entry => {
    //     //     console.log(entry.path);
    //     //     await entry.extract(`${output}/${entry.path}`);
    //     // });
    //     updateStatus('Incompatible Operating Software');
    //     callback('Incompatible Operating Software', null);
    //     return;
    // }

    if (process.platform === 'win32') {
        const extractISOScript = path.join(__dirname, '../../assets/scripts/batch/extract-iso.bat');
        shell.openPath(extractISOScript);

        // copy the iso contents to the jak1 folder created above
        setTimeout(() => {
            try {
                fs.copySync(isoContents, isoFolderPath);
                updateStatus('ISO assests extracted successfully');
                callback(null, 'ISO assests extracted successfully');
                return;
            } catch (err) {
                updateStatus(err);
                callback(err, null);
                return;
            }
        }, 7000);
    } else {
        updateStatus('Linux & Mac support TBD');
        callback('Linux & Mac support TBD', null);
        return;
    }
}

function runDecompiler(callback) {
    let decompScript = null;
    updateStatus('Running the ISO Decompiler');
    if (process.platform === 'win32') {
        decompScript = path.join(jakprojectPath, '/scripts/batch/decomp-jak1.bat');
    } else if (process.platform === 'linux') {
        decompScript = path.join(jakprojectPath, '/scripts/shell/decomp.sh');
    } else if (process.platform === 'darwin') {
        updateStatus('No Mac support for decompiling at this time');
        callback('Mac support TBD', null);
        return;
    }

    if (decompScript) {
        console.log(decompScript);
        updateStatus('Decompiling the ISO File');
        let decomp = exec(decompScript, { shell: true, cwd: path.join(jakprojectPath, '/scripts/batch/') });

        decomp.stdout.on('data', data => {
            console.log(data);
            app.emit('console', data);
        });

        // i see people online saying to use decomp.on('exit'), but thats triggering this listener immediately upon running
        decomp.on('close', (code => {
            console.log('code: ', code);
            if (code === 0) {
                updateStatus('ISO decompiled successfully');
                callback(null, 'ISO decompiled successfully');
                return;
            } else {
                // this else statment isnt actually doing anything because it always returns a code of 0
                updateStatus('ISO decompile failed');
                callback('ISO decompile failed', null);
                return
            }
        }));
    }
}

function isoSeries() {
    series([getISOFile, extractISOFile, runDecompiler, buildGame], (err, result) => {
        if (err) console.log(err);
        if (result) console.log(result);
    });
}

module.exports = {
    isoSeries
}