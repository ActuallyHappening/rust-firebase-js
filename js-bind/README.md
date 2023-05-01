# JS Bind
High level **compile time** binding for JavaScript, using `wasm_bindgen` *runtime* linking.

## Example
<!-- TODO: Document example -->
### `js-bind.toml` config
Custom configuration specific to `js-bind`:
```toml
[modes.example-mode]
file-ref = "your-file-ref"
js-mod-import = "mod_your_porting/sub/modules/allowed"

js-casing = "Camel" # Auto-converts casing if desired

[[files.your-file-ref]]
on-feature = "node-compile-linking"
path = "/relative-to-cargo-toml/final-bundle-cjs.js"

[[files.your-file-ref]]
on-feature = "browser-compile-linking"
path = "/relative-to-cargo-toml/final-bundle-esm.js"
```
### JS specific config
#### `package.json`
You will need at add your own dependancies
```json
{
	// "name": "your-package-name",
	// "version": "1.0.0",
	// "description": "",
	// "scripts": {
	// 	"serve": "trunk serve",
	// 	"js": "rollup -c --watch",
	// 	"test": "./test"
	// },
	// "keywords": [],
	// "author": "",
	// "license": "MIT",
	"devDependencies": {
		"@rollup/plugin-json": "^6.0.0",
		"@rollup/plugin-node-resolve": "^15.0.2",
		"@rollup/plugin-typescript": "^11.1.0", // If using typescript (why not?)
		"rollup": "^3.21.0"
	},
	// "dependencies": {
	// 	"firebase": "^9.20.0" // Example depending on external JS library
	// },
	// "type": "module"
}
```
#### `/js/node-config.js` rollup config
Build `/js/bundle.ts` to `/js/final-bundle-cjs.js`:
```js
import { nodeResolve } from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import json from '@rollup/plugin-json';

export default {
	input: 'js/bundle.ts',
	// watch: true, // Uncomment and run `rollup -c "/js/node-config.js" --watch` to auto-rebuild
	output: {
		file: 'js/final-bundle-cjs.js',
		sourcemap: 'inline',
		format: 'cjs' // Bundles for node compatible js
	},
	plugins: [nodeResolve(), json(), typescript()]
};
```

#### `/js/web-config.js` rollup config
Build `bundle.ts` to `final-bundle-esm.js`:
```js
import { nodeResolve } from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import json from '@rollup/plugin-json';

export default {
	input: 'js/bundle.ts',
	// watch: true, // Uncomment and run `rollup -c "/js/node-config.js" --watch` to auto-rebuild
	output: {
		file: 'js/final-bundle-esm.js',
		sourcemap: 'inline',
		format: 'esm' // Bundles for node compatible js
	},
	plugins: [nodeResolve(), json(), typescript()]
};
```

### `/js/bundle.js` example
This file is **auto generated** for you, based on config found in key `build` in `js-bind.toml` config file:
```ts
function debug_wrapper(func_name: String, mod_name: String, func: any) {
	return function(...args) {
		// Enabled by default, but can disable feature flag to remove
		// #[cfg(feature = "js-debug-logging")]
		console.log(`Calling ${func_name} from ${mod_name} with args:`, args);
		return func(...args);
	}
}

import { yourFunction as _yourFunction } from "mod_your_porting/sub/modules/allowed";
export const yourFunction = debug_wrapper("yourFunction", "mod_your_porting/sub/modules/allowed", _yourFunction);
```

Then, run `cargo js-bind build-js` (install with `cargo install cargo-js-bind`)

```rs
// src/lib.rs
// Using macro
use js_bind::*;

#[js_bind(mode = "example-mode")]
pub fn your_function() -> JsResult<JsString> {
		// ...
}
```