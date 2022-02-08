const { isoSeries } = require('./iso');
const { launchGame, buildGame } = require('./launch');
const { fetchLatestCommit } = require('./gitFetch');

module.exports = {
    launchGame,
    buildGame,
    isoSeries,
    fetchLatestCommit
}