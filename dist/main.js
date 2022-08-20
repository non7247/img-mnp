// access the pre-bundled global API function
const invoke = window.__TAURI__.invoke

var img = new Image();
img.crossOrigin = "anonymous";
img.src = './assets/tabby.JPG';

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
    ctx.drawImage(img, 0, 0);
    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    invoke('img_invert', { pixels: imageData.data }).then(respose => {
        imageData.data = respose;
    });
    ctx.putImageData(imageData, 0, 0);
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
