const webpack = require('webpack');
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin');

module.exports = {
  entry: './src/index.tsx',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        loader: 'ts-loader'
      },
      {
        test: /\.css$/,
        include: path.resolve(__dirname, './src'),
        use: [{
          loader: 'style-loader',
        }, {
          loader: 'css-loader',
          options: {
            modules: true
          },
        }],
      }, {
        test: /\.css$/,
        include: [
          path.resolve(__dirname, './node_modules/monaco-editor'),
          path.resolve(__dirname, './node_modules/ress')
        ],
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /\.ttf$/,
        include: path.resolve(__dirname, './node_modules/monaco-editor'),
        use: ["file-loader"]
      }
    ]
  },
  resolve: {
    alias: {
      src: path.resolve(__dirname, './src/'),
      components: path.resolve(__dirname, './src/components/'),
      containers: path.resolve(__dirname, './src/containers/'),
      modules: path.resolve(__dirname, './src/modules/')
    },
    extensions: ['.tsx', '.ts', '.js'],
  },
  optimization: {
    splitChunks: {
      cacheGroups: {
        monacoCommon: {
          test: /[\\/]node_modules[\\/]monaco\-editor/,
          name: 'monaco-editor-common',
          chunks: 'async'
        }
      }
    }
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './src/index.html',
      filename: 'index.html'
    }),
    new MonacoWebpackPlugin({
      languages: ['scheme']
    })
  ]
};
