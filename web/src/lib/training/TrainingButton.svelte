<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher<{
        start: null;
        stop: null;
        continue: null;
        reset: null;
    }>();

    // [None] -> start -> [Training] -> stop -> [Stopped] -> reset -> [None]
    // [None] -> start -> [Training] -> stop -> [Stopped] -> continue -> [Training]

    enum State {
        None = "none",
        Training = "training",
        Stopped = "stopped",
    }

    let state: State = State.None;

    function on_start() {
        state = State.Training;
        dispatch("start", null);
    }
    function on_stop() {
        state = State.Stopped;
        dispatch("stop", null);
    }
    function on_continue() {
        state = State.Training;
        dispatch("continue", null);
    }
    function on_reset() {
        state = State.None;
        dispatch("reset", null);
    }
    function show(state: boolean) {
        return state ? "" : "display: none;";
    }
</script>

<div style={show(state === State.None)}>
    <button class="primary" on:click={on_start}>Start</button>
</div>
<div style={show(state === State.Training)}>
    <button class="contrast" on:click={on_stop}>Pause</button>
</div>
<div style={show(state === State.Stopped)} class="grid">
    <button class="primary" on:click={on_continue}>Continue</button>
    <button class="secondary" on:click={on_reset}>Reset</button>
</div>
