const path = require('path');
const webpack = require('webpack');
const { merge } = require('webpack-merge');

const base = require('./webpack.base.js');

module.exports = merge(base, {
  mode: 'development',
  devtool: 'inline-source-map',
  devServer: {
    contentBase: path.resolve(__dirname, './public'),
    hot: true,
    port: 8080,
    // Access to `/assets` should resolve (without 404)
    writeToDisk: true,
    before: app => {
      app.get('*.wasm', (req, res, next) => {
        const options = {
          root: path.join(__dirname, 'public/wasm'),
          dotfiles: 'deny',
          headers: {
            'Content-Type': 'application/wasm',
          },
        };
        res.sendFile(req.url, options, err => {
          if (err) {
            console.warn(err);
            next(err);
          }
        });
      });
    },
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader', 'postcss-loader'],
      },
    ],
  },
  plugins: [
    new webpack.DefinePlugin({
      NODE_ENV: '"development"',
    }),
    new webpack.HotModuleReplacementPlugin(),
  ],
});
