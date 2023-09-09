<script lang="ts">
    import GithubLogo from "../assets/github-logo.svelte";
    import ThemeIcon from "../assets/theme-icon.svelte";

    import { onMount } from "svelte";
    import { Chart } from "chart.js";

    let current_theme: "dark" | "light" = "dark";

    // Update chart.js defaults color to callback to update chart when theme updates
    function getStyle(property: string) {
        return () => getComputedStyle(document.body).getPropertyValue(property);
    }
    Chart.defaults.color = getStyle("--color");
    Chart.defaults.borderColor = getStyle("--muted-color");

    function switch_theme() {
        current_theme = current_theme == "dark" ? "light" : "dark";
        set_theme(current_theme);
    }

    function set_theme(theme: "dark" | "light") {
        document.documentElement.setAttribute("data-theme", theme);
        window.localStorage.setItem("data-theme", theme);

        // update chart to refresh the theme
        Object.values(Chart.instances).forEach((chart) => chart.update());
    }

    function get_theme_from_localStorage(): "dark" | "light" | undefined {
        if (typeof window.localStorage != "undefined") {
            const theme = window.localStorage.getItem("data-theme");
            if (theme == "dark" || theme == "light") {
                return theme;
            }
        }
        return undefined;
    }

    function get_preferred_theme() {
        return window.matchMedia("(prefers-color-scheme: dark)").matches
            ? "dark"
            : "light";
    }

    onMount(() => {
        // see if theme is cached in localStorage
        const local_theme = get_theme_from_localStorage();
        if (local_theme != undefined) {
            current_theme = local_theme;
        } else {
            current_theme = get_preferred_theme();
        }
        set_theme(current_theme);
    });
</script>

<nav>
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

<style>
    nav {
        margin: 0 1em;
    }
</style>
