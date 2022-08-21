// access the pre-bundled global API function
const invoke = window.__TAURI__.invoke

var img = new Image();
img.crossOrigin = "anonymous";

var canvas = document.getElementById('canvas');
var ctx = canvas.getContext('2d');

img.onload = function() {
    ctx.drawImage(img, 0, 0);
}

var original = function() {
    console.log("original");
    ctx.drawImage(img, 0, 0);
}

var sepia = function() {
    console.log("sepia");
}

var invert = function() {
    console.log("invert");
}

var grayscale = function() {
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
