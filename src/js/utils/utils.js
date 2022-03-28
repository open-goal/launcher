const { isoSeries } = require('./iso');
const { buildGame, launchGame } = require('./launch');
const { fetchLatestCommit } = require('./gitFetch');

module.exports = {
    launchGame,
    buildGame,
    isoSeries,
    fetchLatestCommit
}