const hamburgerBtn = document.querySelector('#hamburger');
const hamburderIcon = document.querySelector('#hamburger_icon');
const mobileDropDownMenu = document.querySelector('#mobile_menu');
const footerDateSpan = document.querySelector('#date_span');

let hamburgerState = false;

hamburgerBtn.addEventListener("click", function() {
    hamburgerState = !hamburgerState;
    if(hamburgerState) {
        mobileDropDownMenu.classList.replace("hidden", "flex");
        hamburderIcon.classList.replace("fa-bars", "fa-xmark");
    } else {
        mobileDropDownMenu.classList.replace("flex","hidden");
        hamburderIcon.classList.replace("fa-xmark", "fa-bars");
    }
});

footerDateSpan.innerHTML = new Date().getFullYear().toString();