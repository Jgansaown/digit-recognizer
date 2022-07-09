import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
    // development
    if (command === "serve") {
        return {
            plugins: [svelte()],
            server: {
                fs: {
                    // Allow serving files from one level up to the project root
                    allow: [".."],
                },
            },
        };
    }
    // build
    else {
        return {
            plugins: [svelte()],
            server: {
                fs: {
                    // Allow serving files from one level up to the project root
                    allow: [".."],
                },
            },
            // esbuild: {
            //     drop: ['console', 'debugger'],
            // }
        };
    }
});
