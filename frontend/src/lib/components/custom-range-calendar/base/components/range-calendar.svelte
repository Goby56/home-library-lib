<script lang="ts">
	import { box, mergeProps } from "svelte-toolbelt";
	import { type DateValue } from "@internationalized/date";
	import type { RangeCalendarRootProps } from "../types.js";
	import { useRangeCalendarRoot } from "../range-calendar.svelte.js";
	import { noop } from "../../internal/noop.js";
	import { useId } from "bits-ui";
	import { getDefaultDate } from "../../internal/date-time/utils.js";
	import { watch } from "runed";

	let {
		children,
		child,
		id = useId(),
		ref = $bindable(null),
		value = $bindable(),
		onValueChange = noop,
		placeholder = $bindable(),
		onPlaceholderChange = noop,
		weekdayFormat = "narrow",
		weekStartsOn,
		pagedNavigation = false,
		isDateDisabled = () => false,
		isDateUnavailable = () => false,
		fixedWeeks = false,
		numberOfMonths = 1,
		locale = "en",
		calendarLabel = "Event",
		disabled = false,
		readonly = false,
		minValue = undefined,
		maxValue = undefined,
		preventDeselect = false,
		disableDaysOutsideMonth = true,
		onStartValueChange = noop,
		onEndValueChange = noop,
    ranges = [],
    learnMore = undefined,
		...restProps
	}: RangeCalendarRootProps = $props();

	let startValue = $state<DateValue | undefined>(value?.start);
	let endValue = $state<DateValue | undefined>(value?.end);

	const defaultPlaceholder = getDefaultDate({
		defaultValue: value?.start,
	});

	function handleDefaultPlaceholder() {
		if (placeholder !== undefined) return;
		placeholder = defaultPlaceholder;
	}

	// SSR
	handleDefaultPlaceholder();

	watch.pre(
		() => placeholder,
		() => {
			handleDefaultPlaceholder();
		}
	);

	function handleDefaultValue() {
		if (value !== undefined) return;
		value = { start: undefined, end: undefined };
	}

	// SSR
	handleDefaultValue();

	watch.pre(
		() => value,
		() => {
			handleDefaultValue();
		}
	);

	const rootState = useRangeCalendarRoot({
		id: box.with(() => id),
		ref: box.with(
			() => ref,
			(v) => (ref = v)
		),
    ranges: box.with(
      () => ranges,
      (v) => (ranges = v)
    ),
    learnMore: box.with(() => learnMore),
		value: box.with(
			() => value!,
			(v) => {
				value = v;
				onValueChange(v);
			}
		),
		placeholder: box.with(
			() => placeholder!,
			(v) => {
				placeholder = v;
				onPlaceholderChange(v);
			}
		),
		disabled: box.with(() => disabled),
		readonly: box.with(() => readonly),
		preventDeselect: box.with(() => preventDeselect),
		minValue: box.with(() => minValue),
		maxValue: box.with(() => maxValue),
		isDateUnavailable: box.with(() => isDateUnavailable),
		isDateDisabled: box.with(() => isDateDisabled),
		pagedNavigation: box.with(() => pagedNavigation),
		weekStartsOn: box.with(() => weekStartsOn),
		weekdayFormat: box.with(() => weekdayFormat),
		numberOfMonths: box.with(() => numberOfMonths),
		locale: box.with(() => locale),
		calendarLabel: box.with(() => calendarLabel),
		fixedWeeks: box.with(() => fixedWeeks),
		disableDaysOutsideMonth: box.with(() => disableDaysOutsideMonth),
		startValue: box.with(
			() => startValue,
			(v) => {
				startValue = v;
				onStartValueChange(v);
			}
		),
		endValue: box.with(
			() => endValue,
			(v) => {
				endValue = v;
				onEndValueChange(v);
			}
		),
		defaultPlaceholder,
	});

	const mergedProps = $derived(mergeProps(restProps, rootState.props));
</script>

{#if child}
	{@render child({ props: mergedProps, ...rootState.snippetProps })}
{:else}
	<div {...mergedProps}>
		{@render children?.(rootState.snippetProps)}
	</div>
{/if}
