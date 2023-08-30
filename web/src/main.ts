// pico.min.css from https://picocss.com/
import "./pico.min.css";
import "./app.css";
import App from "./App.svelte";

import init from "@wasm/wasm";

let app: App | undefined = undefined;

async function load() {
    const start = performance.now();
    await init();
    const end = performance.now();
    console.log(`wasm init took ${end - start} milliseconds`);

    app = new App({
        target: document.getElementById("app")!,
    });
}

load();

export default app;
