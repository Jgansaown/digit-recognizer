to integrate rust wasm with svelte

1. use package.json's workspace to specify wasm-pack's output as a workspace 
   and svelte project's directory as another workspace
2. in svelte's package.json, specify the wasm as a dependencies
3. in main.ts, load the wasm before svelte
4. now you can use wasm function from svelte
