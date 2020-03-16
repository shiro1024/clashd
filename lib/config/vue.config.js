const ScriptExtHtmlWebpackPlugin = require('script-ext-html-webpack-plugin')
const StyleExtHtmlWebpackPlugin = require('style-ext-html-webpack-plugin')
const webpack = require('webpack')
// const TerserPlugin = require("")

const isDev = process.env.NODE_ENV === 'development'

module.exports = {
	chainWebpack: config => {
		// remove the prefetch plugin
		config.plugins.delete('prefetch')
	},
	configureWebpack: {
		optimization: {
			// minimizer: [new webpack. TerserPlugin()]
		},
		plugins: [
			new webpack.optimize.LimitChunkCountPlugin({
				maxChunks: 1
			}),
			new ScriptExtHtmlWebpackPlugin({ inline: /\.js$/ }),
			isDev ? false : new StyleExtHtmlWebpackPlugin({ inline: /\.css$/ })
		].filter(Boolean)
	}
}
