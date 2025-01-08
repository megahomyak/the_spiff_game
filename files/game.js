/* Reading the following code goes well with
https://soundcloud.com/longlivetheanimalscrew/alk-m-e-llta025-alk-m-e-rave-ep-03-alk-m-e-the-raver
*/

{
    function loadWorld() {
        let worldString = localStorage.getItem("world");
        if (worldString) {
            return JSON.parse(worldString);
        } else {
            return {
                locationName: "test1",
                spoonedItemName: null,
            };
        }
    }

    let world = loadWorld();
    let worldChanged = false;

    function saveWorld() {
        localStorage.setItem("world", JSON.stringify(world));
    }

    function soundNameToSoundPath(soundName) {
        return `game/sounds/${soundName}`;
    }
    function play(soundName) {
        let sound = new Audio();
        sound.src = soundNameToSoundPath(soundName);
        sound.play();
    }

    let screen = document.getElementById("screen");

    let imageDescriptions = {

    };

    function imageNameToImagePath(imageName) {
        return `game/images/${imageName}`;
    }
    function draw(imageName, left, top, right, bottom) {
        let newImage = document.createElement("img");
        newImage.src = imageNameToImagePath(imageName);
        newImage.alt = imageDescriptions[imageName];
        newImage.style.top = `calc(100% * ${top})`;
        newImage.style.left = `calc(100% * ${left})`;
        newImage.style.height = `calc(100% * ${top - bottom})`;
        newImage.style.width = `calc(100% * ${right - left})`;
        screen.appendChild(newImage);
    }

    function switchLocation(newLocationName) {
        worldChanged = true;
        world.locationName = newLocationName;
    }

    function makeLocation(backgroundImageName, fields = {}) {
        return {backgroundImageName, ...fields};
    }

    let locations = {
        test1: makeAwakeLocation("background1"),
    };

    function getCurrentLocation() {
        return locations[world.locationName];
    }

    function drawWorld() {
        let location = getCurrentLocation();
        while (screen.firstChild) {
            screen.removeChild(screen.lastChild);
        }
        draw(location.backgroundImageName, 0, 0, 1, 1);
        if (location.render) {
            location.render();
        }
    }

    function handleInteraction(interactionType) {
        let location = getCurrentLocation();
        if (location[interactionType]) {
            location[interactionType]();
            if (worldChanged) {
                worldChanged = false;
                saveWorld();
                drawWorld();
            }
        }
    }

    for (let interactionType of ["spin", "spoon", "spank", "sprint", "spend", "speedrun"]) {
        let interactionButton = document.getElementById(interactionType);
        interactionButton.addEventListener("click", () => {
            handleInteraction(interactionType);
        });
    }

    /* Asset loading: */

    let soundNames = [];
    let imageNames = [];
    let totalAssetsCount = soundNames.length + imageNames.length;

    let loading = document.getElementById("loading");
    let loaded = document.getElementById("loading");

    let loadedAssetsCount = 0;
    function updateLoading() {
        loading.innerText = `Loading... ${loadedAssetsCount} of ${totalAssetsCount}`;
    }
    updateLoading();

    let loadingFailed = false;
    function handleLoadingError() {
        loadingFailed = true;
        loading.innerText = `Loading error. You can try reloading or contacting the author about this`;
    }
    function handleLoadingSuccess() {
        if (!loadingFailed) {
            ++loadedAssetsCount;
            if (loadedAssetsCount == totalAssetsCount) {
                drawWorld();
                loading.style.display = "none";
                loaded.style.display = null;
            } else {
                updateLoading();
            }
        }
    }

    for (let soundName of soundNames) {
        let sound = new Audio();
        sound.addEventListener("error", handleLoadingError);
        sound.addEventListener("canplaythrough", handleLoadingSuccess);
        sound.src = soundNameToSoundPath(soundName);
    }
    for (let imageName of imageNames) {
        let image = new Image();
        image.addEventListener("error", handleLoadingError);
        image.addEventListener("load", handleLoadingSuccess);
        image.src = imageNameToImagePath(imageName);
    }
}
