const dialogOpen = window.__TAURI__.dialog.open;
const invoke = window.__TAURI__.tauri.invoke;
const convertFileSrc = window.__TAURI__.tauri.convertFileSrc;

const img = new Image();
img.crossOrigin = "anonymous";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

let originalSrc = "";

img.onload = function() {
    ctx.drawImage(img, 0, 0, 500, 392);
}

const original = function() {
    console.log("original");
    invoke('get_original_path').then(response => {
        console.log(response);

        const imgSrc = convertFileSrc(response);
        console.log(imgSrc);

        img.src = imgSrc;
        ctx.drawImage(img, 0, 0, 500, 392);
    });
}

const sepia = function() {
    console.log("sepia");
/*
    invoke('convert_to_sepia').then(response => {
        console.log(response);

        const imgSrc = convertFileSrc(response);
        console.log(imgSrc);

        img.src = imgSrc + "?sepia";
        ctx.drawImage(img, 0, 0, 500, 392);
    });
*/
    img.src = originalSrc;
    ctx.drawImage(img, 0, 0, 500, 392);
    const imageData = ctx.getImageData(0, 0, 500, 392);
    const data = imageData.data;
    const ary = Array.from(data);
    invoke('convert_to_sepia_array', { pixels: ary }).then(response => {
        for (let i = 0; i < response.length; ++i) {
            data[i] = response[i];
        }
        ctx.putImageData(imageData, 0, 0);
    });
}

const invert = function() {
    console.log("invert");
/*
    invoke('convert_to_invert').then(response => {
        console.log(response);

        const imgSrc = convertFileSrc(response);
        console.log(imgSrc);

        img.src = imgSrc + "?invert";
        ctx.drawImage(img, 0, 0, 500, 392);
    });
*/
    img.src = originalSrc;
    ctx.drawImage(img, 0, 0, 500, 392);
    const imageData = ctx.getImageData(0, 0, 500, 392);
    const data = imageData.data;
    const ary = Array.from(data);
    invoke('convert_to_invert_array', { pixels: ary }).then(response => {
        for (let i = 0; i < response.length; ++i) {
            data[i] = response[i];
        }
        ctx.putImageData(imageData, 0, 0);
    });
}

const grayscale = function() {
    console.log("grayscale");
/*
    invoke('convert_to_grayscale').then(response => {
        console.log(response);

        const imgSrc = convertFileSrc(response);
        console.log(imgSrc);

        img.src = imgSrc + "?grayscale";
        ctx.drawImage(img, 0, 0, 500, 392);
    });
*/
    img.src = originalSrc;
    ctx.drawImage(img, 0, 0, 500, 392);
    const imageData = ctx.getImageData(0, 0, 500, 392);
    const data = imageData.data;
    const ary = Array.from(data);
    invoke('convert_to_grayscale_array', { pixels: ary }).then(response => {
        for (let i = 0; i < response.length; ++i) {
            data[i] = response[i];
        }
        ctx.putImageData(imageData, 0, 0);
    });
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

document.getElementById("file_select").addEventListener('click', function() {
    dialogOpen().then(path => {
        if (path) {
            invoke('set_original_path', { path: path.replace(/\\/g,'/') });
            invoke('get_original_path').then(response => {
                console.log(response);
                const imgSrc = convertFileSrc(response);
                console.log(imgSrc);
                img.src = imgSrc;

                originalSrc = imgSrc;
            });
            document.getElementById("original").checked = true;
        }
    });
});
