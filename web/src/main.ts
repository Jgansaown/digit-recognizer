import "./app.css";
import App from "./App.svelte";

import init from "@wasm/wasm";

let app: App | undefined = undefined;

async function load() {
    await init();

    app = new App({
        target: document.getElementById("app")!,
    });
}

load();

export default app;
