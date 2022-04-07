// game title tooltip on hover effect
var tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'));
var tooltipList = tooltipTriggerList.map(function (tooltipTriggerEl) {
    return new bootstrap.Tooltip(tooltipTriggerEl)
});

const body = document.querySelector('body');
const container = document.querySelector('.container');
const background = document.querySelector('#background');
const configButton = document.querySelector("#config");

const { send, receive, handleStatus, handleConsole } = window.electronAPI;

function changeGame(game) {
    switch (game) {
        case "jak1":
            background.src = "../../assets/images/backgrounds/jak1/map.jpg";
            break;
        default:
            background.src = "../../assets/images/backgrounds/jak1/map.jpg";
            break;
    }
}

function config() {
    send('config');
}

function launch() {
    send('launch');
}

handleStatus((event, value) => {
    console.log(value);
});

handleConsole((event, value) => {
    console.log(value);
});