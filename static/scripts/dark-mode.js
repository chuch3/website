// snippet by https://codepen.io/pookagehayes/pen/dyBPjGG

const toggle = document.querySelector(".toggle");
const overlay = document.querySelector(".overlay");

let isDarkModeEnabled = false;

toggle.addEventListener("click", toggleDarkMode);

function toggleDarkMode() {
    isDarkModeEnabled = !isDarkModeEnabled;
    var img = document.getElementsByTagName("img");
    for (let i = 0; i < img.length; i++) {
        img[i].classList.toggle("invert");
    }
    overlay.setAttribute("data-enabled", isDarkModeEnabled)
}
