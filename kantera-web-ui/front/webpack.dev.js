const path = require('path');
const webpackProd = require('./webpack.prod.js');

module.exports = {
  ...webpackProd,
  mode: 'development',
  devtool: 'source-map',
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'build')
  },
  devServer: {
    contentBase: 'build',
    port: 3001,
    hot: true
  }
};
