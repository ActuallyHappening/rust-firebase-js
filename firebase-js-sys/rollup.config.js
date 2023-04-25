import { nodeResolve } from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import json from '@rollup/plugin-json';

export default {
	// the entry point file described above
	input: ['js/app.ts'],
	watch: true,
	// the output for the build folder described above
	output: {
		// file: 'js/*.ts',
		// file: 'firebase-interop.js',
		dir: 'firebase-interop',
		// Optional and for development only. This provides the ability to
		// map the built code back to the original source format when debugging.
		sourcemap: 'inline',
		// Configure Rollup to convert your module code to a scoped function
		// that "immediate invokes". See the Rollup documentation for more
		// information: https://rollupjs.org/guide/en/#outputformat
		format: 'es'
	},
	// Add the plugin to map import paths to dependencies
	// installed with npm
	plugins: [nodeResolve(), json(), typescript()]
};