# Structure of Firebase JS
There are two main modules, which make up the 'top level' module structure:
- `firebase_js::js_semantic`
- `firebase_js::rusty`

These two modules are explored below:

## JS Semantic Module
This module contains functions that inted to accurately reflect the JavaScript API
import structures, while adding a basic layer of types saftey.
If you have a project already implemented in JS and are porting to Rust, this module
is the best place to start.

## Rusty Module
This module applies the powerful `Rust` type system, and functions in this module should
be considered on a higher level of abstraction, they do not accurately map to
'native' functions you would find in the `firebase` JS SDK.


