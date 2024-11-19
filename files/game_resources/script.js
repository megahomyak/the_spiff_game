let imagesToLoad = ["../../comic_pages/1.png"];
let soundsToLoad = [];

let locations = {
    checkingInstructions: {
        render: async () => {
            await drawImage("../../comic_pages/1.png", 0, 0);
        },
        spin: async () => {
            prompt("spin");
        },
        spoon: async () => {
            prompt("spoon");
        },
        spank: async () => {
            prompt("spank");
        },
        sprint: async () => {
            prompt("sprint");
        },
        spend: async () => {
            prompt("spend");
        },
        speedrun: async () => {
            prompt("speerun");
        },
    },
};
let location_ = null;

let state = localStorage.getItem("state");
if (state === null) {
    state = {currentPosition: "checkingInstructions"};
} else {
    state = JSON.parse(state);
}
function commit() {
    localStorage.setItem("state", JSON.stringify(state));
}

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

function switchLocation(locationName) {
    state.currentPosition = locationName;
    commit();
    location_ = locations[locationName];
    (async function() {
        await location_.render();
    })();
}

function runGame() {
    switchLocation(state.currentPosition);
}

let assetsLoaded = 0;
function assetLoaded() {
    ++assetsLoaded;
    if (imagesToLoad.length + soundsToLoad.length == assetsLoaded) {
        runGame();
    }
}
function assetFailedToLoad() {
    resetCanvas();
    writeText(20, "Assets failed to load.", 10, 50);
    writeText(20, "This shouldn't happen.", 10, 80);
    writeText(20, "Please, contact the dev", 10, 110);
    writeText(20, "about it.", 150, 140);
}

for (image of imagesToLoad) {
    preloadImage("game_resources/images/" + image);
}
for (sound of soundsToLoad) {
    preloadSound("game_resources/sounds/" + sound);
}
