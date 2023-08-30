// pico.min.css from https://picocss.com/
import "./pico.min.css";
import "./app.css";

// Svelte App
import App from "./App.svelte";

const app = new App({
    target: document.getElementById("app")!,
});

export default app;
