<script lang="ts">
    import type { Snapshot } from '@sveltejs/kit';
    import SearchView from './SearchView.svelte';
    import type { SearchResponseItem } from '$lib/apis/apelle';

    let {
        onSongChosen: onSongChosenInner,
        query = $bindable('')
    }: {
        onSongChosen?: (s: SearchResponseItem) => void;
        query?: string;
    } = $props();

    let dialog: HTMLDialogElement;

    let searchView: SearchView;

    export function open() {
        dialog.show();
        searchView.search();
    }

    function onSongChosen(s: SearchResponseItem) {
        onSongChosenInner?.(s);
        dialog.close();
    }

    export function close() {
        dialog.close();
    }

    export const snapshot: Snapshot<
        | {
              open: true;
              search: typeof searchView.snapshot extends Snapshot<infer T>
                  ? T
                  : never;
          }
        | {
              open: false;
          }
    > = {
        capture: () =>
            dialog.open
                ? { open: true, search: searchView.snapshot.capture() }
                : { open: false },
        restore: (v) => {
            if (v.open) {
                searchView.snapshot.restore(v.search);
                if (!dialog.open) {
                    dialog.show();
                }
            } else if (!v.open && dialog.open) {
                dialog.close();
            }
        }
    };
</script>

<dialog bind:this={dialog}>
    <SearchView
        bind:this={searchView}
        {onSongChosen}
        onDismiss={close}
        bind:query
    />
</dialog>

<style lang="scss">
    dialog[open] {
        width: 90%;

        display: flex;
        flex-direction: column;

        border-radius: 8px;
        background: #282828;

        color: white;

        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        margin: 0;

        gap: 20px;

        padding: 10px;

        z-index: 20;
    }
</style>
