<script lang="ts">
  import Button from '$lib/components/forms/Button.svelte';
  import type { Snippet } from 'svelte';
  import type { HTMLFormAttributes } from 'svelte/elements';

  interface Props extends Omit<HTMLFormAttributes, 'class'> {
    children?: Snippet;
    submit: Snippet;
    color: 'red' | 'blue';
    submitDisabled?: boolean | null;
  }

  const { children, submit, submitDisabled, color, ...formAttributes }: Props = $props();
</script>

<form
  {...formAttributes}
  class={[
    'm-1 flex flex-col items-center justify-start gap-1 rounded-md border-4 p-5',
    color == 'red' ? 'border-redpill' : 'border-bluepill'
  ]}
>
  {@render children?.()}

  <Button
    disabled={submitDisabled}
    class={['mt-auto w-full', color == 'red' ? 'bg-redpill' : 'bg-bluepill']}
    >{@render submit()}</Button
  >
</form>
