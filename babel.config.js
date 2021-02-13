module.exports = {
  presets: [
    [
      '@babel/preset-env',
      {
        useBuiltIns: 'usage',
        corejs: 3,
        targets: {
          esmodules: true,
        },
        debug: false,
      },
    ],
  ],
  plugins: [
    [
      'babel-plugin-bundled-import-meta',
      {
        bundleDir: 'public/wasm',
        importStyle: 'cjs',
      },
    ],
  ],
};
