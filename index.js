/**
 * Deterministically hash a string into a number
 * @param {string} str
 * @return {number}
 */
const hashString = (str) => {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = (hash << 5) - hash + str.charCodeAt(i);
    hash |= 0; // Convert to 32bit integer
  }
  return hash;
};

const GAME_ID = 'roguelike_game';
const WIDTH = 80;
const HEIGHT = 50;
const CANVAS_SCALE_X = 10;
const CANVAS_SCALE_Y = 16;

class GameRunner {
  constructor(renderMode, container, rngSeed) {
    this.rngSeed = rngSeed;
    this.container = container;
    this.setRenderMode(renderMode);

    // TODO: use babel-transform-class-properties
    this.render = this.render.bind(this);
    this.mouseMove = this.mouseMove.bind(this);
    this.keyDown = this.keyDown.bind(this);

    this.run();
  }

  setRenderMode(renderMode) {
    this.renderMode = renderMode;
    this.vdr = undefined;
    this.canvasCtx = undefined;

    this.container.style.display = 'flex';

    this.el = this.container.querySelector(`#${GAME_ID}`);
    if (this.el) this.el.remove();

    if (renderMode === 'canvas_2d') {
      this.el = document.createElement('canvas');
      this.canvasCtx = this.el.getContext('2d');

      const pixelRatio =
        (window.devicePixelRatio || 1) /
        (this.canvasCtx.webkitBackingStorePixelRatio ||
          this.canvasCtx.mozBackingStorePixelRatio ||
          this.canvasCtx.msBackingStorePixelRatio ||
          this.canvasCtx.oBackingStorePixelRatio ||
          this.canvasCtx.backingStorePixelRatio ||
          1);

      const w = WIDTH * CANVAS_SCALE_X;
      const h = HEIGHT * CANVAS_SCALE_Y;

      this.el.width = w * pixelRatio;
      this.el.height = h * pixelRatio;
      this.el.style.width = w + 'px';
      this.el.style.height = h + 'px';

      this.canvasCtx.font = 'normal 16px monospace';
      this.canvasCtx.textBaseline = 'top';
      this.canvasCtx.imageSmoothingEnabled = false;
      this.canvasCtx.filter = null;

      this.canvasCtx.setTransform(pixelRatio, 0, 0, pixelRatio, 0, 0);
    } else if (renderMode === 'text') {
      this.el = document.createElement('pre');
      this.el.style.lineHeight = 1;
      this.el.style.fontFamily = "'Courier New', Courier, monospace";
    } else if (renderMode === 'html') {
      this.el = document.createElement('div');
      this.el.style.lineHeight = 1;
      this.el.style.fontFamily = "'Courier New', Courier, monospace";
      this.setupVDR();
      this.renderBuffer = new Uint8Array(WIDTH * HEIGHT * 7);
    } else throw new Error(`Unsupported render mode: ${renderMode}`);

    this.el.style.padding = 0;
    this.el.id = GAME_ID;
    this.container.appendChild(this.el);
  }

  async run() {
    const { GameData } = await import('./pkg');

    this.game = new GameData(this.rngSeed);

    this.iter = 0;
    window.requestAnimationFrame(this.render);

    window.addEventListener('mousemove', this.mouseMove);
    window.addEventListener('keydown', this.keyDown);
  }

  // virtual DOM renderer
  setupVDR() {
    this.vdr = [];
    while (this.el.hasChildNodes()) this.el.removeChild(this.el.lastChild);

    let x, y;
    for (y = 0; y < HEIGHT; y++) {
      for (x = 0; x < WIDTH; x++) {
        const $span = document.createElement('span');
        $span.style.backgroundColor = '#000000';
        $span.innerHTML = '&nbsp;';
        this.el.appendChild($span);
        this.vdr.push($span);
      }
      this.el.appendChild(document.createElement('br'));
    }
  }

  render() {
    // adhoc method for slower framerate
    if (this.iter++ % 8 === 0) {
      this.game.tick();

      if (this.canvasCtx)
        this.game.render_to_canvas(
          this.canvasCtx,
          CANVAS_SCALE_X,
          CANVAS_SCALE_Y
        );
      else if (this.vdr) {
        let r = this.renderBuffer;
        this.game.fill_render_buffer(r);

        let x,
          y,
          pixel,
          i = 0,
          j = 0;
        for (y = 0; y < HEIGHT; y++) {
          for (x = 0; x < WIDTH; x++) {
            pixel = this.vdr[i++];

            pixel.style.backgroundColor = `rgb(${r[j]},${r[j + 1]},${
              r[j + 2]
            })`;

            pixel.style.color = `rgb(${r[j + 3]},${r[j + 4]},${r[j + 5]})`;

            pixel.textContent = String.fromCharCode(
              r[j + 6] === 0 || r[j + 6] === 32 ? 160 : r[j + 6]
            );

            j += 7;
          }
        }
      } else this.el.textContent = this.game.render_to_string();
    }

    window.requestAnimationFrame(this.render);
  }

  mouseMove(e) {
    if (this.el.offsetWidth && this.el.offsetHeight) {
      const x = (e.x - this.el.offsetLeft) / this.el.offsetWidth;
      const y = (e.y - this.el.offsetTop) / this.el.offsetHeight;
      if (x >= 0 && y >= 0 && x <= 1 && y <= 1) this.game.move_mouse(x, y);
    }
  }

  keyDown(e) {
    this.game.press_key(e.which);
  }

  dispose() {
    window.removeEventListener('mousemove', this.mouseMove);
    window.removeEventListener('keydown', this.keyDown);

    // TODO: do I need to clear WASM memory?
    // if (this.game) this.game.free();

    this.el.remove();
  }
}

/**
 * @param {object} [options]
 * @param {'text'|'canvas_2d'} [options.renderMode='text']
 * @param {string} [options.seed] Game RNG seed, leave blank for random
 * @param {string} [options.containerId] where to put game, default is document body
 * @return {{ dispose: Function }}
 */
module.exports = (options = {}) => {
  const renderMode = options.renderMode || 'text';
  const container = options.containerId
    ? document.getElementById(options.containerId)
    : document.body;
  const rngSeed = options.seed
    ? hashString(options.seed.toString())
    : Math.floor(Math.random() * 0xffffffff);

  if (options.containerId && !container)
    throw new Error(`Cannot find element with id containerId="${containerId}"`);

  return new GameRunner(renderMode, container, rngSeed);
};
