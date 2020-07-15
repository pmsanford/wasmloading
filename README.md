# wasmloading

Repo with a semi-minimal example of loading a WASM file and executing it as a plugin from a rust program.

Building and usage is documented in the [Makefile](Makefile). Interestingly, WASM loading is _extremely_ slow in dev mode - 60+ seconds on my laptop to JIT the WASM bytecode in dev mode vs 350 _milliseconds_ in release.

Run `make test` and `make test-release` to see the difference. These will also echo a short explanation.
