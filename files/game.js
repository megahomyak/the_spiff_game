/* Reading the following code goes well with
https://soundcloud.com/longlivetheanimalscrew/alk-m-e-llta025-alk-m-e-rave-ep-03-alk-m-e-the-raver
*/

function makeOutputs() {
    return {
        preloadImage(imageName) {

        },
        draw(regions) {
            // Cleanup
            for (let region of regions) {
                region.image
                region.top, region.bottom // etc
                // Put a new element here
            }
        },
        preloadSound(soundName) {

        },
    };
}

function makeEngine(outputs) {
    return {
        handleButtonPress() {
            outputs.
        },
    };
}
