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
