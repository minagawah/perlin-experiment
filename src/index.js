import init, * as PerlinWave from 'perlin-wave';

import './styles.css';

const WASM_PATH =
  NODE_ENV && NODE_ENV === 'production'
    ? 'wasm/perlin-wave/perlin-wave_bg.wasm'
    : void 0;

init(WASM_PATH)
  .then(() => {
    PerlinWave.run({
      color: '#757703', // khaki
      color2: '#d3d626', // mustard
      panels: [
        { id: 'wave' },
      ],
    });
  })
  .catch(err => {
    console.error(err);
  });

if (typeof module.hot !== 'undefined') {
  module.hot.accept();
}
