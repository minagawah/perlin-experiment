# perlin-experiment

A tiny WASM animation using Perlin Noise

![screenshot](screenshot.png)

[View Demo](http://tokyo800.jp/mina/perlin-experiment/)

[1. About](#1-about)  
[2. Installed NPM Packages](#2-installed-npm-packages)  
[3. LICENSE](#3-license)

## 1. About

- Generating an organic looking wave using Perlin Noise
- Click on canvas to switch visualisation modes (wave/equalizer)

Just like
[wasm-pack-canvas-example](https://github.com/minagawah/wasm-pack-canvas-example)
from last time, I am using `wasm-pack` to associate WASM app to Webpack.
For details, refer to
[iced-dynamic-import-sample](https://github.com/minagawah/iced-dynamic-import-sample)
(while it uses `wasm-bindgen` instead of `wasm-pack`).


## 2. Installed NPM Packages

```
yarn add --dev @babel/core @babel/preset-env @babel/cli core-js@3 @babel/runtime-corejs3 babel-loader babel-plugin-bundled-import-meta webpack webpack-cli webpack-dev-server file-loader css-loader style-loader postcss-loader wasm-loader autoprefixer webpack-merge clean-webpack-plugin html-webpack-plugin copy-webpack-plugin mini-css-extract-plugin license-webpack-plugin prettier pretty-quick

# Copy the above to install
```


## 3. License

Dual-licensed under either of the followings.  
Choose at your option.

- The UNLICENSE ([LICENSE.UNLICENSE](LICENSE.UNLICENSE))
- MIT license ([LICENSE.MIT](LICENSE.MIT))
