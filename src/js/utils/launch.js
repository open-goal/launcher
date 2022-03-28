const { app } = require('electron');
const path = require('path');
const { execFile } = require('child_process');
const stream = require('stream');

const userDataPath = app.getPath('userData');
const jakprojectPath = path.join(userDataPath, '/jak-project');

function updateStatus(status) {
    app.emit('status', status);
}

function buildGame(callback) {
    let compilerScript = null;

    if (process.platform === 'win32') {
        compilerScript = path.join(jakprojectPath, '/out/build/Release/bin/goalc.exe');
    } else if (process.platform === 'linux') {
        compilerScript = path.join(jakprojectPath, '/scripts/shell/gc.sh');
    } else if (process.platform === 'darwin') {
        console.log('No Mac support at this time');
        return;
    }

    if (compilerScript) {
        updateStatus('Building the game');
        // so its not console logging the '100%' when i run it in the series, but when i run it on its own its fine.
        // so im going to assume its working properly and its a problem with the way the compiler is outputting the %%%
        // for now i have a timeout that will kill the compiler process after 30 seconds because the compiler should be done by then (twice the length it takes my pc at least)
        let build = execFile(compilerScript, ['-v', '-auto-user'], { timeout: 30000 });
        build.stdout.on('data', data => {
            console.log(data.toString().trim());
            app.emit('console', data);
            if (data.includes('[100%]')) {
                updateStatus('Compiled game successfully!');
                callback(null, 'Compiled game successfully!');
                return;
            }
        });

        build.on('close', () => {
            updateStatus('Compiled game successfully!');
            callback(null, 'Compiled game successfully!');
            return;
        });

        let stdinStream = new stream.Readable();
        stdinStream.push('(mi)');
        stdinStream.push(null);
        stdinStream.pipe(build.stdin);
    }
}

function launchGame() {
    let launchScript = null;

    if (process.platform === 'win32') {
        launchScript = path.join(jakprojectPath, '/out/build/Release/bin/gk.exe');
    } else if (process.platform === 'linux') {
        launchScript = path.join(jakprojectPath, '/scripts/shell/gk.sh');
        return;
    } else if (process.platform === 'darwin') {
        console.log('No Mac support at this time');
        return;
    }
    console.log(launchScript);

    if (launchScript) {
        let launcher = execFile(launchScript, ['-boot', '-fakeiso', '-debug', '-v'], { shell: true });
        launcher.stdout.on('data', data => {
            console.log(data);
            app.emit('console', data);
        });
        return;
    }
}

module.exports = {
    buildGame,
    launchGame
}