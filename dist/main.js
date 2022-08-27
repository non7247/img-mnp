const dialogOpen = window.__TAURI__.dialog.open;
const invoke = window.__TAURI__.tauri.invoke;

const img = new Image();
img.crossOrigin = "anonymous";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

img.onload = function() {
    ctx.drawImage(img, 0, 0);
}

const original = function() {
    console.log("original");
    ctx.drawImage(img, 0, 0);
}

const sepia = function() {
    console.log("sepia");
}

const invert = function() {
    console.log("invert");
}

const grayscale = function() {
    console.log("grayscale");
}

const inputs = document.querySelectorAll('[name=color]');
for (const input of inputs) {
    input.addEventListener("change", function(evt) {
        switch (evt.target.value) {
        case 'inverted':
            return invert();
        case 'grayscale':
            return grayscale();
        case 'sepia':
            return sepia();
        default:
            return original();
        }
    });
}

document.getElementById("target").addEventListener("change", function() {
    const fileList = this.files;
    const blbUrl = window.URL.createObjectURL(fileList[0]);
    console.log(blbUrl);
    img.src = blbUrl;
});

document.getElementById("files").addEventListener('click', function() {
    dialogOpen().then(paths => {
        console.log(paths);
    });
});
