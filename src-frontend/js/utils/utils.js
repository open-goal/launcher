const { isoSeries } = require('./iso');
const { buildGame, launchGame } = require('./launch');
const { fetchMasterRelease } = require('./fetchMasterRelease');

module.exports = {
    launchGame,
    buildGame,
    isoSeries,
    fetchMasterRelease
}