# `npm:firebase@9` in Rust!
This is a Rust port of the Firebase JS SDK. It is currently a work in progress, and is under heavy development.

## Why?
I want to use Firebase, and 100% Rust. Therefore, being experienced with the JS SDK,
this package tries to port the ease of use and simplicity of the JS SDK while
not just using a custom HTTP implementation, having the full power of JavaScript at your
fingertips!

This package tries to maintain the same modular structure of the Firebase JS SDK,
while converting the APIs into Rust.
Pure JS examples are given for each Rust function

## Usage
(assuming secrets.rs file)
```rust
use firebase_js::{app::initialize_app, database::{get_database, on_value_changed, get_ref_of_root}};
use log::info;

use crate::secrets::URL;

mod secrets;

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	info!("firebase-js: main.rs()");
	
	let app = initialize_app(&secrets::get_config()).ok().unwrap();

	let db = get_database(&app, URL.to_string()).ok().unwrap();

	let reference = get_ref_of_root(&db).ok().unwrap();

	on_value_changed(&reference, &move |_| {
		info!("RS: on_value_changed() WOW!")
	})
}
```

## WIP
- Testing:
	- [] Literally no tests :(
- App:
	- [x] initialize_app
- Database:
	- [x] get_database
	- [x] get_ref
		- [x] get_ref_of_root (get_ref but for root of db)
	- [x] on_value_changed