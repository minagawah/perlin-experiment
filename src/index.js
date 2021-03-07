import init, * as PerlinWave from 'perlin-wave';

import './styles.css';

const WASM_PATH =
  NODE_ENV && NODE_ENV === 'production'
    ? 'wasm/perlin-wave/perlin-wave_bg.wasm'
    : void 0;

const CANVAS_PANELS = [
  {
    id: 'control',
    ratio: 15.0 / 1.0,
    color: '#c0e822',
  },
  {
    id: 'wave',
    ratio: 3.0 / 1.0,
    color: '#759203',
  },
];

document.addEventListener('DOMContentLoaded', onload);

if (typeof module.hot !== 'undefined') {
  module.hot.accept();
}

function onload() {
  init(WASM_PATH)
    .then(() => {
      PerlinWave.run({
        bgcolor: '#222',
        panels: CANVAS_PANELS.reduce(panelsReducer, []),
      });
    })
    .catch(err => {
      console.error(err);
    });
}

function panelsReducer(acc = [], { id, ratio, color }) {
  const key = `#${id}`;
  const el = document.querySelector(key);
  if (el) {
    const width = (el.offsetWidth || 0).toFixed(1);
    const height = (width / ratio).toFixed(1);
    acc.push({
      id,
      color,
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
