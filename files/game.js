/* Reading the following code goes well with
https://soundcloud.com/longlivetheanimalscrew/alk-m-e-llta025-alk-m-e-rave-ep-03-alk-m-e-the-raver
*/

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

function play(soundName) {
//TODO
}

function draw(imageName, lowleft, lowright, topleft, topright) {
//TODO
}

function switchLocation(newLocationName) {
    worldChanged = true;
    world.locationName = newLocationName;
}

function drawWorld() {
//TODO
// * Cleanup
// * Adding new <img>s
}

function handleInteraction(interactionType) {
    let loc = locations[world.locationName];
    if (loc[interactionType]) {
        loc[interactionType]();
        if (worldChanged) {
            saveWorld();
            drawWorld();
        }
    }
}
