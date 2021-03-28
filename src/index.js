import init, * as PerlinWave from 'perlin-wave';

import './styles.css';

const WASM_PATH =
  NODE_ENV && NODE_ENV === 'production'
    ? 'wasm/perlin-wave/perlin-wave_bg.wasm'
    : void 0;

const COLOR = '#c0e822';
const COLOR_DARK = '#759203';

const APP_CONFIG = {
  bgcolor: '#222',
  panels: [
    {
      id: 'control',
      ratio: 15.0 / 1.0,
      color: COLOR,
      color2: COLOR_DARK,
    },
    {
      id: 'wave',
      ratio: 3.0 / 1.0,
      color: COLOR,
      color2: COLOR_DARK,
    },
  ].reduce(panelsReducer, []),
};

document.addEventListener('DOMContentLoaded', () => {
  init(WASM_PATH)
    .then(() => {
      PerlinWave.run(APP_CONFIG);
    })
    .catch(err => {
      console.error(err);
    });
});

if (typeof module.hot !== 'undefined') {
  module.hot.accept();
}

function panelsReducer(acc = [], { id, ratio, color, color2 }) {
  const key = `#${id}`;
  const el = document.querySelector(key);
  if (el) {
    const width = (el.offsetWidth || 0).toFixed(1);
    const height = (width / ratio).toFixed(1);
    acc.push({
      id,
      color,
      color2,
      width,
      height,
    });

    // The wrapper element for canvas produces
    // a weird gap underneath the canvas,
    // and we need to get rid of that.
    el.style.height = `${height}px`;
  }
  return acc;
}
