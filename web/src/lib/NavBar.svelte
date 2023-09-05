<script lang="ts">
    import { onMount } from "svelte";

    import GithubLogo from "../assets/github-logo.svelte";
    import ThemeIcon from "../assets/theme-icon.svelte";

    let current_theme = "dark";

    onMount(() => {
        current_theme = window.matchMedia("(prefers-color-scheme: dark)")
            .matches
            ? "dark"
            : "light";

        document.documentElement.setAttribute("data-theme", current_theme);
    });

    function switch_theme() {
        current_theme = current_theme == "dark" ? "light" : "dark";

        document.documentElement.setAttribute("data-theme", current_theme);
    }
</script>

<nav style="margin: 0 1em">
    <ul>
        <li><strong>Digit Recognizer</strong></li>
    </ul>
    <ul>
        <li>
            <a
                class="contrast"
                href="https://github.com/Jgansaown/digit-recognizer"
                ><GithubLogo /></a
            >
        </li>
        <li>
            <button
                on:click={switch_theme}
                style="padding: 0; background: none; border: none; box-shadow: none; color: inherit"
                ><ThemeIcon /></button
            >
        </li>
    </ul>
</nav>
