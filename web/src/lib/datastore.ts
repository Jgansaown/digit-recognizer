import { writable, Writable } from "svelte/store";

export const dataset: Writable<{
    data: Uint8Array | null,
    label: Uint8Array| null
}> = writable({
    data: null,
    label: null,
});