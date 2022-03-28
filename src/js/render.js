var tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'))
var tooltipList = tooltipTriggerList.map(function (tooltipTriggerEl) {
    return new bootstrap.Tooltip(tooltipTriggerEl)
});

const body = document.querySelector('body');
const container = document.querySelector('.container');
const offcanvas = document.querySelector(".offcanvas-start");
let backdrop = document.querySelector('.offcanvas-backdrop');
const background = document.querySelector('#background');
const logo = document.querySelector('#logo');

const { recieve, send } = window.api;

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
        console.log(target);
    } else {
        changeGame(target);
    }
}

function changeGame(game) {
    switch (game) {
        case "jak1":
            background.src = "../assets/images/jak1-bg.png";
            logo.src = "../assets/images/jak1-logo.png";
            break;
        case "jak2":
            background.src = "../assets/images/jak2-bg.png";
            logo.src = "../assets/images/jak-2.png";
            break;
        case "jak3":
            background.src = "../assets/images/jak3-bg.png";
            logo.src = "../assets/images/jak-3.png";
            break;
        default:
            background.src = "../assets/images/jak1-bg.png";
            logo.src = "../assets/images/jak1-logo.png";
            break;
    }
}