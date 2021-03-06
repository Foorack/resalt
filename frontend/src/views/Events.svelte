<script lang="ts">
    import { onMount } from "svelte";
    import { Table } from "sveltestrap";
    import Icon from "../components/Icon.svelte";
    import { AlertType, getEvents, showAlert } from "../controller";
    import { theme } from "../stores";
    import { useNavigate } from "svelte-navigator";
    import TablePaginate from "../components/TablePaginate.svelte";
    import { writable } from "svelte/store";
    const navigate = useNavigate();

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const events = writable(null);
    const expandedEvents = writable<string[]>([]);

    function toggleExpandEvent(index: string) {
        console.log(index);
        if ($expandedEvents.includes(index)) {
            expandedEvents.update((array) => array.filter((i) => i !== index));
        } else {
            expandedEvents.update((array) => [...array, index]);
        }
        console.log($expandedEvents); // TODO: remove
    }

    function updateData() {
        getEvents(paginationSize, (paginationPage - 1) * paginationSize)
            .then((data) => {
                events.set(
                    data.map((event) => {
                        const data = JSON.parse(
                            event.data ?? "{data: {}}"
                        ).data;
                        return {
                            ...event,
                            jid: data.jid ?? "",
                            target: data.id ?? "",
                            fun: data.fun ?? "",
                            data_parsed: data,
                            data_formatted: JSON.stringify(data, null, 2),
                            unique_index: (
                                (event.tag ?? "") +
                                "_" +
                                (event.timestamp ?? "")
                            ).replace(/ /g, "_"),
                        };
                    })
                );
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Failed fetching events", err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

<h1>Events</h1>

<div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
    <Table
        dark={$theme.dark}
        hover
        id="eventListTable"
        class="b-0 mb-0 {$theme.dark ? 'text-light border-secondary' : ''}"
    >
        <thead
            class="bg-dark border-0 {$theme.dark ? 'text-light' : 'text-white'}"
        >
            <tr>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">Tag</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="15" />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Function</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="12" />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Target</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="12" />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Job ID</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="15" />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Date</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                    </div>
                </th>
            </tr>
        </thead>
        <tbody class="align-middle">
            {#if $events == null}
                <p>Loading</p>
            {:else if $events.length == 0}
                <div class="p-3">No events exist. Very unusal.</div>
            {:else}
                {#each $events as event}
                    <tr>
                        <!-- <th scope="row">{event.id}</th> -->
                        <td
                            on:click={() =>
                                toggleExpandEvent(event.unique_index)}
                            class="mouse-pointer"
                        >
                            <Icon
                                size="1.125"
                                name={$expandedEvents.includes(
                                    event.unique_index
                                )
                                    ? "chevron-up"
                                    : "chevron-down"}
                            />
                            {event.tag}
                        </td>
                        <td>{event.fun}</td>
                        <td>{event.target}</td>
                        <td>{event.jid}</td>
                        <td><small>{event.timestamp}</small></td>
                    </tr>
                    {#if $expandedEvents.includes(event.unique_index)}
                        <tr>
                            <td
                                class={$theme.dark
                                    ? "bg-secondary"
                                    : "bg-light"}
                                colspan="5"
                            >
                                <pre
                                    class="text-left">{event.data_formatted}</pre>
                            </td>
                        </tr>
                    {/if}
                {/each}
            {/if}
        </tbody>
    </Table>
</div>

<TablePaginate
    bind:size={paginationSize}
    bind:page={paginationPage}
    last={$events == null || $events.length < paginationSize}
    {updateData}
/>
