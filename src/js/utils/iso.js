const { dialog, app, shell } = require('electron');
const fs = require('fs-extra');
const path = require('path');
const { series } = require('async');
const { exec } = require('child_process');
const { buildGame } = require('./launch');
const { extract } = require('./extract');

const userDataPath = app.getPath('userData');
const isoSavePath = path.join(userDataPath, '/iso');
const jakprojectPath = path.join(userDataPath, '/jak-project');

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
    series([getISOFile, extract, runDecompiler, buildGame], (err, result) => {
        if (err) console.log(err);
        if (result) console.log(result);
    });
}

module.exports = {
    isoSeries
}