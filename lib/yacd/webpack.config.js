'use strict';

const path = require('path');
const webpack = require('webpack');
const TerserPlugin = require('terser-webpack-plugin');
const HTMLPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const ScriptExtHtmlWebpackPlugin = require('script-ext-html-webpack-plugin');

const pkg = require('./package.json');

process.env.BABEL_ENV = process.env.NODE_ENV;
const isDev = process.env.NODE_ENV !== 'production';

const postcssPlugins = () =>
  [
    require('postcss-import')(),
    require('postcss-simple-vars')(),
    require('postcss-custom-media')({
      importFrom: [
        {
          customMedia: {
            '--breakpoint-not-small': 'screen and (min-width: 30em)',
            '--breakpoint-medium':
              'screen and (min-width: 30em) and (max-width: 60em)',
            '--breakpoint-large': 'screen and (min-width: 60em)'
          }
        }
      ]
    }),
    require('postcss-nested')(),
    require('autoprefixer')(),
    require('postcss-extend-rule')(),
    isDev ? false : require('cssnano')()
  ].filter(Boolean);

module.exports = {
  // https://webpack.js.org/configuration/devtool/
  devtool: isDev ? 'eval-source-map' : false,
  entry: {
    app: ['./src/app.js']
  },
  output: {
    path: path.resolve(__dirname, 'public')
  },
  mode: isDev ? 'development' : 'production',
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules((?!react-tiny-fab).)*$/,
        use: { loader: 'babel-loader', options: { cacheDirectory: true } }
      },
      {
        test: /\.(ttf|eot|woff|woff2)(\?.+)?$/,
        use: [{ loader: 'file-loader', options: { name: '[name].[ext]' } }]
      },
      {
        test: /\.css$/,
        exclude: /\.module\.css$/,
        use: [
          { loader: 'style-loader' },
          { loader: 'css-loader' },
          { loader: 'postcss-loader', options: { plugins: postcssPlugins } }
        ].filter(Boolean)
      },
      {
        test: /\.module\.css$/,
        use: [
          { loader: 'style-loader' },
          {
            loader: 'css-loader',
            options: {
              modules: {
                localIdentName: isDev
                  ? '[path]_[name]_[local]_[hash:base64:5]'
                  : '[hash:base64:10]'
              }
            }
          },
          {
            loader: 'postcss-loader',
            options: { plugins: postcssPlugins }
          }
        ].filter(Boolean)
      }
    ]
  },
  optimization: {
    minimizer: [new TerserPlugin()]
  },
  plugins: [
    new HTMLPlugin({
      title: 'yacd - Yet Another Clash Dashboard',
      template: 'src/index.template.ejs',
      inject: true,
      filename: 'index.html'
    }),
    new webpack.DefinePlugin({
      __DEV__: JSON.stringify(isDev),
      __VERSION__: JSON.stringify(pkg.version),
      'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV)
    }),
    new CleanWebpackPlugin(),
    new webpack.IgnorePlugin(/^\.\/locale$/, /moment$/),
    new webpack.ProvidePlugin({
      TextDecoder: ['text-encoding', 'TextDecoder'],
      TextEncoder: ['text-encoding', 'TextEncoder']
    }),
    new ScriptExtHtmlWebpackPlugin({ inline: /\.js$/ })
  ]
};
