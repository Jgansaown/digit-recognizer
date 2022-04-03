<script lang="ts">
    export let value: number = 0;
    export let min: number;
    export let max: number;
    export let text: string = "";
</script>

<div>
    <label for="slider">{text}</label>
    <input id="slider" type="range" bind:value {min} {max} />
</div>

<style>
    div {
        display: flex;
        align-items: center;
    }
</style>
