<script lang="ts">
    export let type = "regular";
    export let name = "bx-rocket";
    export let size = "2";
    let cssClass = "";
    export { cssClass as class };
    export let htmlElement = null;
    export let style = "";

    $: full_name =
        (type === "solid" ? "bxs-" : type === "logo" ? "bxl-" : "bx-") + name;
</script>

<i
    bind:this={htmlElement}
    on:click
    class="bx {full_name} {cssClass.indexOf('align-') == -1
        ? 'align-middle'
        : ''} {cssClass}"
    style="font-size: {size}rem;{style}"
/>
