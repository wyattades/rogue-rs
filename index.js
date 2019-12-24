import('./pkg')
  .then((wasm) => {
    console.log('Running game!');

    let mode;
    let el;
    const setMode = (_mode) => {
      mode = _mode;
      document.querySelectorAll('[data-game]').forEach((el) => {
        el.style.display = 'none';
      });
      el = document.querySelector(`[data-game="${mode}"]`);
      el.style.display = 'block';
    };

    document.getElementById('render_mode').addEventListener('change', (e) => {
      setMode(e.target.value);
    });
    setMode('text');

    const ctx = document.querySelector('canvas').getContext('2d');
    ctx.font = 'normal 12px monospace';

    const game = wasm.GameData.new();

    let i = 0;
    const renderLoop = () => {
      if (i++ % 8 === 0) {
        game.tick();
        if (mode === 'text') el.textContent = game.render_to_string();
        else game.render_to_canvas(ctx);
      }

      requestAnimationFrame(renderLoop);
    };
    requestAnimationFrame(renderLoop);

    document.addEventListener('mousemove', (e) => {
      if (el.offsetWidth && el.offsetHeight) {
        const x = (e.x - el.offsetLeft) / el.offsetWidth;
        const y = (e.y - el.offsetTop) / el.offsetHeight;
        game.move_mouse(x, y);
      }
    });

    document.addEventListener('keydown', (e) => {
      game.press_key(e.which);
    });
  })
  .catch(console.error);
