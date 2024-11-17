let imagesToLoad = ["blah.png"];
let soundsToLoad = [];

function preloadImage(filePath) {
    let image = new Image();
    image.addEventListener("load", assetLoaded);
    image.addEventListener("error", assetFailedToLoad);
    image.src = filePath;
}
function preloadSound(filePath) {
    let sound = new Audio();
    sound.autoplay = false;
    sound.addEventListener("canplaythrough", assetLoaded);
    sound.addEventListener("error", assetFailedToLoad);
    sound.src = filePath;
}

let canvas = document.getElementById("canvas");

let ctx = canvas.getContext("2d");
function resetCanvas() {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

resetCanvas();
function writeText(fontSize, text, x, y) {
    ctx.fillStyle = "white";
    ctx.font = fontSize + "px sans-serif";
    ctx.fillText(text, x, y);
}
writeText(48, "Loading...", 20, 110);

function drawImage(path, x, y) {
    return new Promise((resolve, reject) => {
        let image = new Image();
        image.addEventListener("load", () => {
            ctx.drawImage(image, x, y);
            resolve();
        });
        image.addEventListener("error", () => reject());
        image.src = path;
    });
}

let assetsLoaded = 0;
function assetLoaded() {
    if (imagesToLoad.length + soundsToLoad.length == assetsLoaded) {
        runGame();
    }
    ++assetsLoaded;
}
function assetFailedToLoad() {
    resetCanvas();
    writeText(20, "Assets failed to load.", 10, 50);
    writeText(20, "This shouldn't happen.", 10, 80);
    writeText(20, "Please, contact the dev", 10, 110);
    writeText(20, "about it.", 150, 140);
}

for (image of imagesToLoad) {
    preloadImage(image);
}
for (sound of soundsToLoad) {
    preloadSound(sound);
}
