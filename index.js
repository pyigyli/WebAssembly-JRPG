const rust = import('./pkg/jrpg_wasm_engine');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext('webgl', {antialias: true});

rust.then(m => {
  if (!gl) {
    alert('Failed to initialize WebGL');
    return;
  }

  const FPS_THROTTLE = 1000.0 / 30.0;
  var lastDrawTime = -1;

  const gameClient = new m.GameClient();

  const sprites = document.getElementsByTagName('img');
  for (let i = 0; i < document.getElementsByTagName('img').length; i++) {
    gameClient.add_sprite(sprites.item(i).src.split('resources/')[1].split('.png')[0], sprites.item(i));
  }

  function render() {
    window.requestAnimationFrame(render);
    const currentTime = Date.now();

    if (currentTime >= lastDrawTime + FPS_THROTTLE) {
      lastDrawTime = currentTime;
      gameClient.update();
      gameClient.render();
    }
  }

  render();
})