<template>
	<div>
		<section class="container">
			<el-input v-model="rules_url">
				<template slot="prepend">上次更新： {{ timeago }}</template>
			</el-input>
			<el-button type="primary" @click="onSubmit">立即创建</el-button>
		</section>
	</div>
</template>

<script>
import { format } from 'timeago.js'
export default {
	data() {
		return {
			rules_url: '',
			last_update: 0
		}
	},
	computed: {
		timeago() {
			return format(this.last_update, 'zh_CN')
		}
	},
	methods: {
		onSubmit() {
			console.log('submit!')
		}
	}
}
</script>

<style lang="stylus">
html {
	font-family: 'Source Sans Pro', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
	font-size: 16px;
	word-spacing: 1px;
	-ms-text-size-adjust: 100%;
	-webkit-text-size-adjust: 100%;
	-moz-osx-font-smoothing: grayscale;
	-webkit-font-smoothing: antialiased;
	box-sizing: border-box;
}

*, *:before, *:after {
	box-sizing: border-box;
	margin: 0;
}

.button--green {
	display: inline-block;
	border-radius: 4px;
	border: 1px solid #3b8070;
	color: #3b8070;
	text-decoration: none;
	padding: 10px 30px;
}

.button--green:hover {
	color: #fff;
	background-color: #3b8070;
}

.button--grey {
	display: inline-block;
	border-radius: 4px;
	border: 1px solid #35495e;
	color: #35495e;
	text-decoration: none;
	padding: 10px 30px;
	margin-left: 15px;
}

.button--grey:hover {
	color: #fff;
	background-color: #35495e;
}

.container {
	margin: 0 auto;
	min-height: 100vh;
	display: flex;
	justify-content: center;
	align-items: center;
	text-align: center;

	& > div, & > button {
		margin: 1em;
	}

	& > button {
		margin-left: 0;
	}
}
</style>
