const rust = import('./pkg/index');
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

  document.getElementById('soundtrackSlider').onchange = (event) => gameClient.set_soundtrack_volume(event.target.value / 10);
  document.getElementById('sfxSlider').onchange = (event) => gameClient.set_sfx_volume(event.target.value / 10);

  const sprites = document.getElementsByTagName('img');
  for (let i = 0; i < sprites.length; i++) {
    gameClient.add_sprite(sprites.item(i).src.split('resources/')[1].split('.png')[0], sprites.item(i));
  }

  const audios = document.getElementsByTagName('audio');
  for (let i = 0; i < audios.length; i++) {
    if (audios.item(i).src.includes('soundtracks')) {
      gameClient.add_soundtrack(audios.item(i));
    } else {
      gameClient.add_sfx(audios.item(i));
    }
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