// game title tooltip on hover effect
var tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'));
var tooltipList = tooltipTriggerList.map(function (tooltipTriggerEl) {
    return new bootstrap.Tooltip(tooltipTriggerEl)
});

const body = document.querySelector('body');
const container = document.querySelector('.container');
const offcanvas = document.querySelector(".offcanvas-start");
let backdrop = document.querySelector('.offcanvas-backdrop');
const background = document.querySelector('#background');
const configButton = document.querySelector("#config");

const { send, receive, handleStatus, handleConsole } = window.electronAPI;

function openNav() {
    container.style.marginLeft = "100px";
    offcanvas.classList.add("show");
    backdrop = document.querySelector('.offcanvas-backdrop');
}

function closeNav() {
    container.style.marginLeft = "0";
    offcanvas.classList.remove("show");
    offcanvas.style.visibility = "hidden";
    offcanvas.ariaHidden = "true";
    offcanvas.ariaModal = "false";
    backdrop.classList.remove('show');
    body.style.removeProperty('overflow');
    body.style.removeProperty('padding');
}

function navBarClick(target) {
    closeNav();
    if (target.includes("settings")) {
        send('settings');
    } else {
        changeGame(target);
    }
}

function changeGame(game) {
    switch (game) {
        case "jak1":
            background.src = "../assets/images/backgrounds/jak1/map.jpg";
            break;
        default:
            background.src = "../assets/images/backgrounds/jak1/map.jpg";
            break;
    }
}

function config() {
    send('getISO');
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