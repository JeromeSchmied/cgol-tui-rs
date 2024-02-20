import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new(64, 64);

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
