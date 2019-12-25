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

class GameRunner {
  constructor(renderMode, container, rngSeed) {
    this.rngSeed = rngSeed;
    container.style.display = 'flex';

    this.el = container.querySelector(`#${GAME_ID}`);
    if (this.el) this.el.remove();

    if (renderMode === 'canvas_2d') {
      this.el = document.createElement('canvas');
      this.el.width = 800;
      this.el.height = 500;
      this.canvasCtx = this.el.getContext('2d');
      this.canvasCtx.font = 'normal 12px monospace';
    } else {
      this.el = document.createElement('pre');
      this.el.style.padding = 0;
      this.el.style.lineHeight = 1;
      this.el.style.fontFamily = "'Courier New', Courier, monospace";
    }

    this.el.id = GAME_ID;
    container.appendChild(this.el);

    this.render = this.render.bind(this);
    this.mouseMove = this.mouseMove.bind(this);
    this.keyDown = this.keyDown.bind(this);

    this.run();
  }

  async run() {
    const { GameData } = await import('./pkg');

    this.game = GameData.new(this.rngSeed);

    this.iter = 0;
    window.requestAnimationFrame(this.render);

    window.addEventListener('mousemove', this.mouseMove);
    window.addEventListener('keydown', this.keyDown);
  }

  render() {
    // adhoc method for slower framerate
    if (this.iter++ % 8 === 0) {
      this.game.tick();
      if (this.canvasCtx) this.game.render_to_canvas(this.canvasCtx);
      else this.el.textContent = this.game.render_to_string();
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
