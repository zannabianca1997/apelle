<script lang="ts">
    import type { ComponentProps } from 'svelte';
    import Button from './Button.svelte';
    import TextInput from './TextInput.svelte';

    interface Props
        extends Pick<
            ComponentProps<typeof TextInput>,
            'label' | 'placeholder' | 'error' | 'value' | 'onchange' | 'oninput'
        > {
        submitTxt: string;
    }

    let {
        label,
        placeholder,
        submitTxt,
        error,
        value = $bindable(),
        onchange,
        oninput
    }: Props = $props();
</script>

<div class="flex w-full items-center gap-3">
    <TextInput
        {label}
        {placeholder}
        {error}
        {onchange}
        {oninput}
        bind:value
        class="alig flex-grow-1"
    />
    <Button>{submitTxt}</Button>
</div>
