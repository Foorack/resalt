<script lang="ts">
    import { Table } from "sveltestrap";
    import JsonViewer from "../../components/JsonViewer.svelte";
    import { theme } from "../../stores";

    export let minion;
    let rawData = false;

    $: pkgs = JSON.parse(minion.pkgs);
</script>

{#if !minion.pkgs}
    <div class="p-3">No packages data. Please refresh minion.</div>
{:else}
    <button
        class="btn btn-light float-end border border-1 rounded-none"
        style="margin-top: -0rem;z-index: 4;position: absolute;right: 0;"
        on:click={() => (rawData = !rawData)}
    >
        {rawData ? "View List" : "View JSON"}
    </button>
    {#if rawData}
        <JsonViewer code={minion.pkgs} />
    {:else}
        <div class="p-3">
            <div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
                <Table
                    dark={$theme.dark}
                    hover
                    class="b-0 mb-0 {$theme.dark
                        ? 'text-light border-secondary'
                        : ''}"
                >
                    <thead
                        class="bg-dark border-0 {$theme.dark
                            ? 'text-light'
                            : 'text-white'}"
                    >
                        <tr>
                            <th scope="col" class="border-secondary">
                                Package
                            </th>
                            <th scope="col" class="border-secondary">
                                Version
                            </th>
                            <th scope="col" class="border-secondary" />
                        </tr>
                    </thead>
                    <tbody>
                        {#each Object.entries(pkgs) as pkg}
                            <tr>
                                <td>{pkg[0]}</td>
                                <td>{pkg[1]}</td>
                                <td />
                            </tr>
                        {/each}
                    </tbody>
                </Table>
            </div>
        </div>
    {/if}
{/if}
