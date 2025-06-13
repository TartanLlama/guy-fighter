.PHONY: js-plugin

js-plugin: guy-fighter-install/plugins/js-plugin.wasm

guy-fighter-install/plugins/js-plugin.wasm: js-plugin/build/js-plugin.wasm
	@mkdir -p guy-fighter-install/plugins
	@cp js-plugin/build/js-plugin.wasm guy-fighter-install/plugins/js-plugin.wasm

js-plugin/build/js-plugin.wasm: js-plugin/plugin.js
	@mkdir -p js-plugin/build
	componentize-js js-plugin/plugin.js --wit guy-fighter/wit/guy-fighter.wit --world-name guy-fighter-plugin -d stdio -d random -d clocks -d http -o js-plugin/build/js-plugin.wasm