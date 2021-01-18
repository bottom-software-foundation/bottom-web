wasm-pack build ./bottom-wasm/ --target no-modules
cp -r ./bottom-wasm/pkg/* ./site/
