<script lang="ts">
    import type {
        ChangeEventHandler,
        ClassValue,
        HTMLInputAttributes
    } from 'svelte/elements';

    interface CapturedProps {
        label?: string | null;

        password?: boolean;
        error?: string | null;

        class?: ClassValue | null;

        onchange?: ChangeEventHandler<HTMLInputElement> | undefined | null;
        noerror?: boolean;
    }

    type Props = CapturedProps &
        Omit<HTMLInputAttributes, keyof CapturedProps | 'type' | 'id'>;

    const id = $props.id();
    let {
        label,

        password = false,
        error: errorTxt = $bindable(null),

        noerror = false,

        value = $bindable(),

        onchange: onchangeInner,

        class: clazz,

        ...inputAttributes
    }: Props = $props();

    let dirty: boolean = $state(false);

    function onchange(...args: Parameters<NonNullable<typeof onchangeInner>>) {
        dirty = true;
        onchangeInner?.(...args);
    }

    let error: string | null = $derived((dirty && errorTxt) || null);
</script>

<div class={['flex flex-col gap-1.5', clazz]}>
    {#if label}
        <label
            for="input-{id}"
            class="w-full text-base leading-[150%] font-light tracking-[1%]"
        >
            {label}
        </label>
    {/if}
    <input
        id="input-{id}"
        type={password ? 'password' : 'text'}
        class="w-full rounded-md border border-[#122a42] p-3 text-base leading-[150%] font-light tracking-[1%] text-black placeholder-[#122a4282]"
        bind:value
        {onchange}
        {...inputAttributes}
    />
    {#if !noerror}
        <div
            class={[
                'mt-auto h-[20px] w-full',
                !!error &&
                    'rounded-sm border border-red-500 text-center text-xs leading-[150%] font-light tracking-[1%] text-red-500'
            ]}
        >
            {error}
        </div>
    {/if}
</div>
