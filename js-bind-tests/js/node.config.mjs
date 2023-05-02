import { nodeResolve } from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import json from '@rollup/plugin-json';

export default {
	input: 'js/bundle.ts',
	watch: true,
	output: {
		// file: 'firebase-interop/bundle-cjs.js',
		file: 'js/bundle-cjs.js',
		sourcemap: 'inline',
		format: 'cjs'
	},
	plugins: [nodeResolve(), json(), typescript()]
};