
Run `wasm-pack build --target web` to build.

The results go into the "pkg" dir, which is gitignored.

There are html files in the "www" dir which use the wasm files.
You may need to run a local webserver to use them (due to CORS);
or, see comments in those files about disabling the relevant parts of CORS.
