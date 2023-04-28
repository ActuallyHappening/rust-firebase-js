# Firebase Types
Implementation agnostic types for 'native' `firebase` types in Rust.
These types try to reflect the types you would use in the official firebase SDKs.

## Installation
### Cargo
Install using cargo like:
```bash
cargo add firebase-types
```

By default, this package ships with features `serde` but not `expose-jsvalue`.
I (the author) am using this package to implement `firebase-js`, interopping with JS,
which is the reason for the `expose-jsvalue`.
You can obviously disable `serde` if you so desire:
```bash
cargo add firebase-types --no-default-features
```

### `git`
Clone the repo like so:
```bash
git clone https://github.com/ActuallyHappening/rust-firebase-js/
```
Then, you can establish a local dependency for more fine-grained control:
```toml
[dependancies.firebase-types]
path = "../path/to/repo/firebase-types"
version = "X.Y.Z" # Optional for publishing, relies on `crates.io` version instead of local
```