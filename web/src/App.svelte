<script lang="ts">
    import ModelTrainer from "./lib/ModelTrainer.svelte";
    import NavBar from "./lib/NavBar.svelte";
</script>

<NavBar />

<main class="container">
    <article>
        <h2>Train your own model!</h2>
        <ModelTrainer />
    </article>
</main>
