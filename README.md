<pre>
╔══════════════════════════════════════════════════════════════════════════════════════╗
║                                                                                      ║
║  ██████╗ ██╗   ██╗██╗   ██╗    ███████╗██╗ ██████╗ ██╗  ██╗████████╗███████╗██████╗  ║
║ ██╔════╝ ██║   ██║╚██╗ ██╔╝    ██╔════╝██║██╔════╝ ██║  ██║╚══██╔══╝██╔════╝██╔══██╗ ║
║ ██║  ███╗██║   ██║ ╚████╔╝     █████╗  ██║██║  ███╗███████║   ██║   █████╗  ██████╔╝ ║
║ ██║   ██║██║   ██║  ╚██╔╝      ██╔══╝  ██║██║   ██║██╔══██║   ██║   ██╔══╝  ██╔══██╗ ║
║ ╚██████╔╝╚██████╔╝   ██║       ██║     ██║╚██████╔╝██║  ██║   ██║   ███████╗██║  ██║ ║
║  ╚═════╝  ╚═════╝    ╚═╝       ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝ ║
║                                                                                      ║
║                                                                                      ║ 
║                                                                                      ║
║                                🥊 ULTIMATE EDITION 🥊                               ║
╚══════════════════════════════════════════════════════════════════════════════════════╝
</pre>

A demonstration of a plugin system using [WASM Components](https://component-model.bytecodealliance.org/).

Guy Fighter is a console application that lets you simulate fights between different types of guys. Plugins can be written and loaded into Guy Fighter to invent entirely new types of guys to fight.

## Overview

Guy Fighter consists of four components:

- A [host console application](https://github.com/TartanLlama/guy-fighter/tree/main/guy-fighter), written in Rust
- A [Deluxe Dog plugin](https://github.com/TartanLlama/guy-fighter/tree/main/c-plugin), written in C
- A [Coffee Guy plugin](https://github.com/TartanLlama/guy-fighter/tree/main/js-plugin), written in JavaScript
- An [interface definition file](https://github.com/TartanLlama/guy-fighter/blob/main/guy-fighter/wit/guy-fighter.wit) that defines the communication between these components, written in [WIT](https://component-model.bytecodealliance.org/design/wit.html)

The host application embeds the [Wasmtime](https://github.com/bytecodealliance/wasmtime) WASM runtime. The plugins are compiled to WASM components and run inside the WASM sandbox, communicating with the host through a limited interface defined in the WIT file.

## Dependencies

- [wasi-sdk](https://github.com/WebAssembly/wasi-sdk) for building the C plugin
- [NodeJS](https://nodejs.org/en/download) for building the JS plugin
- [ComponentizeJS](https://github.com/bytecodealliance/ComponentizeJS) for building the JS plugin
- [Rust](https://www.rust-lang.org/tools/install) for building host application

## Running

```bash
$ make run
```

This builds the host application, C and JS plugins, and runs Guy Fighter.
