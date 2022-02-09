const download = require('download-git-repo');
// const fetch = require('node-fetch');
// const Store = require('electron-store');
const { app } = require('electron');

// const store = new Store();
const userDataPath = app.getPath('userData');

// download the github repo
async function downloadRepo() {
    const repoURL = "direct:https://github.com/water111/jak-project.git"

    download(repoURL, `${userDataPath}/jak-project`, { clone: true }, function (err) {
        console.log(err ? err : 'Success');
        return;
    });
}

// check the sha of the latest commit
async function fetchLatestCommitSha() {
    const commitURL = "https://api.github.com/repos/water111/jak-project/commits/master";

    const response = await fetch(commitURL);
    let data = await response.json();
    return data.sha;
}

// fetch the latest commit
async function fetchLatestCommit() {
    const latestSha = await fetchLatestCommitSha();
    // store.set('latest_sha', latestSha);
    const currentSha = store.get('current_sha');

    if (latestSha === currentSha) {
        return 'NO UPDATES REQUIRED: You have the latest download from the Jak-Project github';
    }

    await downloadRepo()
        // .then((() => store.set('current_sha', latestSha)))
        .then(() => {
            return 'Downloaded the latest updates from github!';
        })
        .catch(err => {
            console.log(err)
        });
}

module.exports = {
    fetchLatestCommit
}