<script lang="ts">
    import { theme } from "../stores";
    import Icon from "./Icon.svelte";

    export let size: number;
    export let page: number;
    export let last: boolean;
    export let updateData: Function;

    function paginateIncrement() {
        if (last) {
            return;
        }
        page++;
        updateData();
    }
    function paginateDecrement() {
        if (page === 1) {
            return;
        }
        page--;
        updateData();
    }
    function paginateFirst() {
        page = 1;
        updateData();
    }
    function setSize(newSize: number) {
        size = newSize;
        updateData();
    }
</script>

<div class="nav bg-dark w-100 justify-content-start no-select">
    <div
        class="nav-link fw-bold mouse-pointer {page === 1
            ? 'text-secondary'
            : 'text-white'}"
        on:click={paginateFirst}
    >
        &lt;&lt;
    </div>
    <div
        class="nav-link fw-bold mouse-pointer {page === 1
            ? 'text-secondary'
            : 'text-white'}"
        on:click={paginateDecrement}
    >
        &lt;
    </div>
    <div
        class="nav-link fw-bold mouse-pointer {last
            ? 'text-secondary'
            : 'text-white'}"
        on:click={paginateIncrement}
    >
        &gt;
    </div>
    <div class="nav-item dropdown ms-3">
        <span
            class="nav-link text-white mouse-pointer"
            id="dropdownPaginatePageSize"
            role="button"
            data-bs-toggle="dropdown"
            aria-expanded="false"
        >
            Page size ({size})
            <Icon name="caret-down" size="1.125" />
        </span>
        <ul
            class="dropdown-menu dropdown-menu-dark ms-5 {$theme.dark
                ? 'bg-secondary'
                : 'bg-darker'}"
            aria-labelledby="dropdownPaginatePageSize"
        >
            {#each [10, 20, 50, 100, 250] as s}
                <li>
                    <span
                        class="dropdown-item mouse-pointer {$theme.dark
                            ? 'text-light'
                            : 'text-white'} {size === s ? 'fw-bold' : ''}"
                        on:click={() => setSize(s)}>{s}</span
                    >
                </li>
            {/each}
        </ul>
    </div>
    <div class="nav-link text-muted">
        <small>
            Showing page {page}
        </small>
    </div>
</div>

<style>
    .dropdown-menu {
        min-width: auto;
    }
</style>
