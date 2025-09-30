<script lang="ts">
    import type { Snapshot } from '@sveltejs/kit';
    import SearchView from './SearchView.svelte';
    import type { SearchResponseItem } from '$lib/apis/apelle';

    const {
        onSongChosen: onSongChosenInner
    }: {
        onSongChosen?: (s: SearchResponseItem) => void;
    } = $props();

    let dialog: HTMLDialogElement;

    let searchView: SearchView;

    export async function open(initialQuery: string) {
        dialog.show();
        await searchView.searchFor(initialQuery);
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

<dialog
    bind:this={dialog}
    class="flex w-[90%] flex-col gap-[39px] rounded-lg bg-[#282828] text-white"
>
    <SearchView bind:this={searchView} {onSongChosen} onDismiss={close} />
</dialog>
