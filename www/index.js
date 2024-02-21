import {
    Universe,
    Cell,
    featherweigth_spaceship,
    gosper_glider_gun,
    copperhead,
} from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 6; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
// const universe = Universe.new(128, 128);
// const universe = Universe.from_figur(128, 128, featherweigth_spaceship());
// const universe = Universe.from_figur(128, 128, gosper_glider_gun());
const universe = Universe.from_figur(128, 128, copperhead());
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

canvas.addEventListener("click", (event) => {
    const boundRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundRect.width;
    const scaleY = canvas.height / boundRect.height;

    const canvasLeft = (event.clientX - boundRect.left) * scaleX;
    const canvasTop = (event.clientY - boundRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    // universe.toggle
    universe.toggle_cell(row, col);
    drawGrid();
    drawCells();
});

const ctx = canvas.getContext("2d");

let animId = null;

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "||";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "|>";
    cancelAnimationFrame(animId);
    animId = null;
};

playPauseButton.addEventListener("click", (event) => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

const renderLoop = () => {
    debugger;
    console.log("Tickling");

    universe.tick();

    drawGrid();
    drawCells();

    animId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
    return animId === null;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE,
            );
        }
    }

    ctx.stroke();
};

drawGrid();
drawCells();
// requestAnimationFrame(renderLoop);
play();

// const pre = document.getElementById("game-of-life-canvas");
// const universe = Universe.new(64, 64);

// const renderLoop = () => {
//     pre.textContent = universe.render();
//     universe.tick();

//     requestAnimationFrame(renderLoop);
// };

// requestAnimationFrame(renderLoop);
