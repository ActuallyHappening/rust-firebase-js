import { nodeResolve } from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import json from '@rollup/plugin-json';

export default {
	input: 'js/bundle.ts',
	watch: true,
	output: {
		// file: 'firebase-interop/bundle-es.js',
		file: 'js/bundle-esm.js',
		sourcemap: 'inline',
		format: 'esm'
	},
	plugins: [nodeResolve(), json(), typescript()]
};