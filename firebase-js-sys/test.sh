# if first CLI arg is "copy", then copy all files in tests/*.rs to tests/web/*_web.rs and append "web" to the end of the file name, and append the line wasm_test!(run_in_browser)
# if [ "$1" = "copy" ]; then
		for file in tests/*.rs; do
				filename=$(basename -- "$file")
				filename="${filename%.*}"
				cp $file tests/web/${filename}_web.rs
				echo "wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);" >> tests/web/${filename}_web.rs
		done
# fi

wasm-pack test --headless --safari --no-default-features --features web-not-node
wasm-pack test --node --no-default-features --features node-not-web
open /Applications/Visual\ Studio\ Code.app