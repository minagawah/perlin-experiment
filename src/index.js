import init, * as PerlinWave from 'perlin-wave';

import './styles.css';

const WASM_PATH =
  NODE_ENV && NODE_ENV === 'production'
    ? 'wasm/perlin-wave/perlin-wave_bg.wasm'
    : void 0;

const CANVAS_IDS = ['control', 'wave'];

const CANVAS_RATIO = {
  control: 15.0 / 1.0,
  wave: 3.0 / 1.0,
};

document.addEventListener('DOMContentLoaded', onload);

if (typeof module.hot !== 'undefined') {
  module.hot.accept();
}

function onload() {
  init(WASM_PATH)
    .then(() => {
      PerlinWave.run({
        color: '#757703',
        color2: '#d3d626',
        panels: CANVAS_IDS.reduce(panelsReducer, []),
      });
    })
    .catch(err => {
      console.error(err);
    });
}

function panelsReducer(acc = [], id) {
  const key = `#${id}`;
  const el = document.querySelector(key);
  if (el) {
    const width = el.offsetWidth || 0;
    const height = width / CANVAS_RATIO[id];
    acc.push({ id, width, height });

    // The wrapper element for canvas produces
    // a weird gap underneath the canvas,
    // and we need to get rid of that.
    el.style.height = `${height}px`;
  }
  return acc;
}
