cargo build --target wasm32-unknown-unknown --example simple_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/simple_web.wasm --out-dir docs/simple --target web
cargo build --target wasm32-unknown-unknown --example advanced_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/advanced_web.wasm --out-dir docs/advanced --target web
cargo build --target wasm32-unknown-unknown --example manual_implement_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/manual_implement_web.wasm --out-dir docs/manual_implement --target web
cargo build --target wasm32-unknown-unknown --example shared_data_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/shared_data_web.wasm --out-dir docs/shared_data --target web
rem cargo build --target wasm32-unknown-unknown --example datepicker_web --release --features="datepicker"
rem wasm-bindgen target/wasm32-unknown-unknown/release/examples/datepicker_web.wasm --out-dir docs/datepicker --target web
cargo build --target wasm32-unknown-unknown --example nalgebra_glm_web --release --features="nalgebra_glm"
wasm-bindgen target/wasm32-unknown-unknown/release/examples/nalgebra_glm_web.wasm --out-dir docs/nalgebra_glm --target web
rem cargo build --target wasm32-unknown-unknown --example filepicker_web --release --features="filepicker"
rem wasm-bindgen target/wasm32-unknown-unknown/release/examples/filepicker_web.wasm --out-dir docs/filepicker --target web
cargo build --target wasm32-unknown-unknown --example hashmap_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/hashmap_web.wasm --out-dir docs/hashmap --target web
