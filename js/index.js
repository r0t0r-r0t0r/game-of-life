"use strict";

const settings = {
    pixelWidth: 3,
    pixelHeight: 3,

    bgColor: '#232323',
    fgColor: '#dbd580',
    gridColor: '#848484',

    xPixelCount: 300,
    yPixelCount: 300,

    grid: false,
};

function getScreenSize({xPixelCount, pixelWidth, yPixelCount, pixelHeight}) {
    return [xPixelCount * pixelWidth, yPixelCount * pixelHeight];
}

function getFieldSize({xPixelCount, yPixelCount}) {
    return [xPixelCount, yPixelCount];
}

function createScreen(settings) {
    const screenElement = document.getElementById('screen');

    const [screenWidth, screenHeight] = getScreenSize(settings);

    screenElement.setAttribute("width", screenWidth.toString());
    screenElement.setAttribute("height", screenHeight.toString());

    const screen = screenElement.getContext('2d');

    return screen;
}

function createBgDrawer(settings, screen) {
    const {bgColor} = settings;
    const [width, height] = getScreenSize(settings);
    return {
        draw() {
            screen.fillStyle = bgColor;
            screen.fillRect(0, 0, width, height);
        }
    }
}

function createFieldDrawer(settings, screen, getCell) {
    const [screenWidth, screenHeight] = getScreenSize(settings);
    const {xPixelCount, yPixelCount, pixelWidth, pixelHeight, fgColor, gridColor} = settings;

    function drawCell(x, y) {
        screen.fillStyle = fgColor;
        screen.fillRect(x * pixelWidth + 1, y * pixelHeight + 1, pixelWidth - 2, pixelHeight - 2);
    }

    function drawGrid() {
        screen.strokeStyle = gridColor;
        screen.beginPath();
        for (let i = 0; i <= xPixelCount; i++) {
            screen.moveTo(i * pixelWidth, 0);
            screen.lineTo(i * pixelWidth, screenHeight);
        }
        for (let i = 0; i <= yPixelCount; i++) {
            screen.moveTo(0, i * pixelHeight);
            screen.lineTo(screenWidth, i * pixelHeight);
        }
        screen.stroke();
    }

    function gridEnabled() {
            drawGrid();
            for (let y = 0; y < yPixelCount; y++) {
                for (let x = 0; x < xPixelCount; x++) {
                    if (getCell(x, y)) {
                        drawCell(x, y)
                    }
                }
            }
    }

    function gridDisabled() {
        for (let y = 0; y < yPixelCount; y++) {
            for (let x = 0; x < xPixelCount; x++) {
                if (getCell(x, y)) {
                    screen.fillStyle = fgColor;
                    screen.fillRect(x * pixelWidth, y * pixelHeight, pixelWidth, pixelHeight);
                }
            }
        }
    }

    return {
        draw: settings.grid ? gridEnabled : gridDisabled,
    }
}

function createBuf([xPixelCount, yPixelCount]) {
    const lines = new Array(yPixelCount);
    for (let i = 0; i < lines.length; i++) {
        const line = new Array(xPixelCount).fill(false);
        lines[i] = line;
    }

    return lines;
}

function copyBuf([xPixelCount, yPixelCount], destination, source) {
    for (let y = 0; y < yPixelCount; y++) {
        for (let x = 0; x < xPixelCount; x++) {
            destination[y][x] = source[y][x];
        }
    }
}

function createBufBuilder(settings) {
    const size = getFieldSize(settings);
    const buf = createBuf(size);

    function placeBlinker(x, y) {
        buf[y][x] = true;
        buf[y][x + 1] = true;
        buf[y][x + 2] = true;
    }
    
    function placeGlider(x, y) {
        fillRect(x, y, 3, 3, false);
        buf[y][x + 2] = true;
        buf[y + 1][x] = true;
        buf[y + 1][x + 2] = true;
        buf[y + 2][x + 1] = true;
        buf[y + 2][x + 2] = true;
    }

    function placePentadecathlon(x, y) {
        fillRect(x, y, 3, 8, true);

        buf[y + 1][x + 1] = false;
        buf[y + 6][x + 1] = false;
    }

    function placeAcorn(x, y) {
        fillRect(x, y, 7, 3, false);
        buf[y][x + 1] = true;
        buf[y + 1][x + 3] = true;
        buf[y + 2][x] = true;
        buf[y + 2][x + 1] = true;
        buf[y + 2][x + 4] = true;
        buf[y + 2][x + 5] = true;
        buf[y + 2][x + 6] = true;
    }

    function fillRandom(x = 0, y = 0, width = size[0], height = size[1]) {
        for (let cx = x; cx < x + width; cx++) {
            for (let cy = y; cy < y + height; cy++) {
                buf[cy][cx] = Math.random() > 0.5;
            }
        }
    }

    function fillRect(x, y, width, height, value) {
        for (let cx = x; cx < x + width; cx++) {
            for (let cy = y; cy < y + height; cy++) {
                buf[cy][cx] = value;
            }
        }
    }

    return {
        placeBlinker,
        placeGlider,
        placePentadecathlon,
        placeAcorn,
        fillRect,
        fillRandom,
        build: () => buf,
    }
}

function createField(settings, initialBuf) {
    const fieldSize = getFieldSize(settings);
    const [xPixelCount, yPixelCount] = fieldSize;

    let currFrame = createBuf(fieldSize);
    copyBuf(fieldSize, currFrame, initialBuf);

    let nextFrame = createBuf(fieldSize);

    function getLiveNeighbourCount(y, x) {
        const prevX = x > 0 ? x - 1 : xPixelCount - 1;
        const nextX = x < (xPixelCount - 1) ? x + 1 : 0;
        const prevY = y > 0 ? y - 1 : yPixelCount - 1;
        const nextY = y < (yPixelCount - 1) ? y + 1 : 0;

        function boolToNum(v) {
            return v ? 1 : 0;
        }

        return boolToNum(currFrame[prevY][x]) +
            boolToNum(currFrame[prevY][nextX]) +
            boolToNum(currFrame[y][nextX]) +
            boolToNum(currFrame[nextY][nextX]) +
            boolToNum(currFrame[nextY][x]) +
            boolToNum(currFrame[nextY][prevX]) +
            boolToNum(currFrame[y][prevX]) +
            boolToNum(currFrame[prevY][prevX]);
    }

    function getLiveNeighbourCount1(y, x) {
        const prevX = x - 1;
        const nextX = x + 1;
        const prevY = y - 1;
        const nextY = y + 1;

        function boolToNum(v) {
            return v ? 1 : 0;
        }

        return boolToNum(prevY >= 0 && currFrame[prevY][x]) +
            boolToNum(prevY >= 0 && nextX < xPixelCount && currFrame[prevY][nextX]) +
            boolToNum(nextX < xPixelCount && currFrame[y][nextX]) +
            boolToNum(nextY < yPixelCount && nextX < xPixelCount && currFrame[nextY][nextX]) +
            boolToNum(nextY < yPixelCount && currFrame[nextY][x]) +
            boolToNum(nextY < yPixelCount && prevX >= 0 && currFrame[nextY][prevX]) +
            boolToNum(prevX >= 0 && currFrame[y][prevX]) +
            boolToNum(prevY >= 0 && prevX >= 0 && currFrame[prevY][prevX]);
    }

    return {
        getCell(x, y) {
            return currFrame[y][x];
        },
        step() {
            for (let y = 0; y < yPixelCount; y++) {
                for (let x = 0; x < xPixelCount; x++) {
                    const isDead = !currFrame[y][x];
                    const neighbourCount = getLiveNeighbourCount(y, x);
        
                    let nextAlive = false;
        
                    if (isDead) {
                        if (neighbourCount === 3) {
                            nextAlive = true;
                        }
                    } else {
                        if (neighbourCount < 2) {
                            nextAlive = false;
                        } else if (neighbourCount === 2 || neighbourCount === 3) {
                            nextAlive = true;
                        } else {
                            nextAlive = false;
                        }
                    }
        
                    nextFrame[y][x] = nextAlive;
                }
            }

            [currFrame, nextFrame] = [nextFrame, currFrame];
        }
    }
}

function start() {
    const bufBuilder = createBufBuilder(settings);

    // bufBuilder.placeBlinker(10, 10);
    // bufBuilder.placeGlider(1, 1);
    // bufBuilder.placePentadecathlon(30, 5);

    // bufBuilder.placeAcorn(150, 150);
    //bufBuilder.fillRect(100, 100, 100, 100, true);

    // bufBuilder.fillRandom();

    const border = 3;
    bufBuilder.fillRect(border, border, settings.xPixelCount - 2*border, settings.yPixelCount - 2*border, true);

    const initalFieldState = bufBuilder.build();

    const field = createField(settings, initalFieldState);

    const screen = createScreen(settings);

    const bgDrawer = createBgDrawer(settings, screen);
    const fieldDrawer = createFieldDrawer(settings, screen, field.getCell);

    const drawers = [bgDrawer, fieldDrawer];


    const framesForStep = 1;
    let frameCounter = 0;

    function step(timestamp) {
        for (const drawer of drawers) {
            drawer.draw();
        }

        frameCounter++;
        if (frameCounter >= framesForStep) {
            field.step();
            frameCounter = 0;
        }

        requestAnimationFrame(step);
    }

    step(0);
    // setInterval(step, 50);
    // requestAnimationFrame(step);
}

start();
