.PHONY: c-plugin

c-plugin: c-plugin/build/c-plugin.wasm
	@mkdir -p guy-fighter-install/plugins
	@cp c-plugin/build/c-plugin.wasm guy-fighter-install/plugins/c-plugin.wasm

c-plugin/gen/plugin.c: guy-fighter/wit/guy-fighter.wit
	@mkdir -p c-plugin/gen
	wit-bindgen c guy-fighter/wit -w plugin --out-dir c-plugin/gen --autodrop-borrows=yes

c-plugin/build/c-plugin.wasm: c-plugin/plugin.c c-plugin/gen/plugin.c
	@mkdir -p c-plugin/build
	/opt/wasi-sdk/bin/clang c-plugin/plugin.c c-plugin/gen/plugin.c c-plugin/gen/plugin_component_type.o -o c-plugin/build/c-plugin-module.wasm -mexec-model=reactor
	wasm-tools component new c-plugin/build/c-plugin-module.wasm -o c-plugin/build/c-plugin.wasm
	
