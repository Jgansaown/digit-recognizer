<script lang="ts">
    import { onMount } from "svelte";

    import ModelTrainer from "./lib/ModelTrainer.svelte";

    onMount(() => {
        // document.documentElement.setAttribute("data-theme", "light");
        // document.documentElement.setAttribute("data-theme", "dark");
    });
</script>

<main class="container">
    <h1>Digit Recognizer</h1>

    <article>
        <h2>Train your own model!</h2>
        <ModelTrainer />
    </article>
</main>
