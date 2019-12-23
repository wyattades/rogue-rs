const $el = document.getElementById('game');

import('./pkg')
  .then((wasm) => {
    console.log('Running game!');

    // const pressedKeys = new Set();
    const game = wasm.GameData.new();

    let i = 0;
    const renderLoop = () => {
      if (i++ % 8 === 0) {
        game.tick();
        $el.textContent = game.render();
      }

      requestAnimationFrame(renderLoop);
    };
    requestAnimationFrame(renderLoop);

    document.addEventListener('mousemove', (e) => {
      const x = (e.x - $el.offsetLeft) / $el.offsetWidth;
      const y = (e.y - $el.offsetTop) / $el.offsetHeight;
      game.move_mouse(x, y);
    });

    // let keysAllowed = {};
    document.addEventListener('keydown', (e) => {
      // pressedKeys.add(e.which);

      // if (keysAllowed[e.which] === false) return;
      // keysAllowed[e.which] = false;

      game.press_key(e.which);
    });
    // document.addEventListener('keyup', (e) => {
    //   keysAllowed[e.which] = true;

    //   pressedKeys.delete(e.which);
    // });
    // document.addEventListener('focus', () => {
    //   keysAllowed = {};
    //   pressedKeys.clear();
    // });
  })
  .catch(console.error);
