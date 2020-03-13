import pkg from "./package";
import ScriptExtHtmlWebpackPlugin from "script-ext-html-webpack-plugin";
import webpack from "webpack";

export default {
	mode: "spa",
	generate: {
		fallback: false
	},

	/*
	 ** Headers of the page
	 */
	head: {
		title: pkg.name,
		meta: [
			{ charset: "utf-8" },
			{
				name: "viewport",
				content: "width=device-width, initial-scale=1"
			},
			{
				hid: "description",
				name: "description",
				content: pkg.description
			}
		]
	},

	/*
	 ** Customize the progress-bar color
	 */
	loading: { color: "#fff" },

	/*
	 ** Global CSS
	 */
	css: ["element-ui/lib/theme-chalk/index.css"],

	/*
	 ** Plugins to load before mounting the App
	 */
	plugins: ["@/plugins/element-ui"],

	/*
	 ** Nuxt.js modules
	 */
	modules: [],

	/*
	 ** Build configuration
	 */
	build: {
		transpile: [/^element-ui/],
		splitChunks: {
			layouts: false,
			pages: false,
			commons: false
		},
		optimization: {
			runtimeChunk: false,
			splitChunks: {
				cacheGroups: {
					default: false
				}
			}
		},
		/*
		 ** You can extend webpack config here
		 */
		extend(config, ctx) {
			config.plugins.push(
				new webpack.optimize.LimitChunkCountPlugin({
					maxChunks: 1
				}),
				new ScriptExtHtmlWebpackPlugin({ inline: /\.js$/ })
			);
		}
	}
};
