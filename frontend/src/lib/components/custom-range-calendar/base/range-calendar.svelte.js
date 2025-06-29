import { getLocalTimeZone, isSameDay, isSameMonth, isToday, } from "@internationalized/date";
import { useRefById } from "svelte-toolbelt";
import { Context, watch } from "runed";
import { CalendarRootContext } from "../calendar/calendar.svelte.js";
import { useId } from "bits-ui";
import { getAriaDisabled, getAriaSelected, getDataDisabled, getDataSelected, getDataUnavailable, } from "../internal/attrs.js";
import { getAnnouncer } from "../internal/date-time/announcer.js";
import { createFormatter } from "../internal/date-time/formatter.js";
import { createMonths, getCalendarElementProps, getCalendarHeadingValue, getIsNextButtonDisabled, getIsPrevButtonDisabled, getWeekdays, handleCalendarKeydown, handleCalendarNextPage, handleCalendarPrevPage, shiftCalendarFocus, useEnsureNonDisabledPlaceholder, useMonthViewOptionsSync, useMonthViewPlaceholderSync, } from "../internal/date-time/calendar-helpers.svelte.js";
import { areAllDaysBetweenValid, getDateValueType, isAfter, isBefore, isBetweenInclusive, toDate, } from "../internal/date-time/utils.js";

export class RangeCalendarRootState {
    opts;
    months = $state([]);
    visibleMonths = $derived.by(() => this.months.map((month) => month.value));
    announcer;
    formatter;
    accessibleHeadingId = useId();
    focusedValue = $state(undefined);
    lastPressedDateValue = undefined;
    constructor(opts) {
        this.opts = opts;
        this.announcer = getAnnouncer();
        this.formatter = createFormatter(this.opts.locale.current);
        useRefById(opts);
        this.months = createMonths({
            dateObj: this.opts.placeholder.current,
            weekStartsOn: this.opts.weekStartsOn.current,
            locale: this.opts.locale.current,
            fixedWeeks: this.opts.fixedWeeks.current,
            numberOfMonths: this.opts.numberOfMonths.current,
        });
        $effect(() => {
            if (this.formatter.getLocale() === this.opts.locale.current)
                return;
            this.formatter.setLocale(this.opts.locale.current);
        });
        /**
         * Updates the displayed months based on changes in the placeholder values,
         * which determines the month to show in the calendar.
         */
        useMonthViewPlaceholderSync({
            placeholder: this.opts.placeholder,
            getVisibleMonths: () => this.visibleMonths,
            weekStartsOn: this.opts.weekStartsOn,
            locale: this.opts.locale,
            fixedWeeks: this.opts.fixedWeeks,
            numberOfMonths: this.opts.numberOfMonths,
            setMonths: this.setMonths,
        });
        /**
         * Updates the displayed months based on changes in the options values,
         * which determines the month to show in the calendar.
         */
        useMonthViewOptionsSync({
            fixedWeeks: this.opts.fixedWeeks,
            locale: this.opts.locale,
            numberOfMonths: this.opts.numberOfMonths,
            placeholder: this.opts.placeholder,
            setMonths: this.setMonths,
            weekStartsOn: this.opts.weekStartsOn,
        });
        /**
         * Update the accessible heading's text content when the `fullCalendarLabel`
         * changes.
         */
        $effect(() => {
            const node = document.getElementById(this.accessibleHeadingId);
            if (!node)
                return;
            node.textContent = this.fullCalendarLabel;
        });
        /**
         * Synchronize the start and end values with the `value` in case
         * it is updated externally.
         */
        watch(() => this.opts.value.current, (value) => {
            if (value.start && value.end) {
                this.opts.startValue.current = value.start;
                this.opts.endValue.current = value.end;
            }
            else if (value.start) {
                this.opts.startValue.current = value.start;
                this.opts.endValue.current = undefined;
            }
            else if (value.start === undefined && value.end === undefined) {
                this.opts.startValue.current = undefined;
                this.opts.endValue.current = undefined;
            }
        });
        /**
         * Synchronize the placeholder value with the current start value
         */
        watch(() => this.opts.value.current, (value) => {
            const startValue = value.start;
            if (startValue && this.opts.placeholder.current !== startValue) {
                this.opts.placeholder.current = startValue;
            }
        });
        watch([() => this.opts.startValue.current, () => this.opts.endValue.current], ([startValue, endValue]) => {
            if (this.opts.value.current &&
                this.opts.value.current.start === startValue &&
                this.opts.value.current.end === endValue) {
                return;
            }
            if (startValue && endValue) {
                this.#updateValue((prev) => {
                    if (prev.start === startValue && prev.end === endValue) {
                        return prev;
                    }
                    if (isBefore(endValue, startValue)) {
                        const start = startValue;
                        const end = endValue;
                        this.#setStartValue(end);
                        this.#setEndValue(start);
                        return { start: endValue, end: startValue };
                    }
                    else {
                        return {
                            start: startValue,
                            end: endValue,
                        };
                    }
                });
            }
            else if (this.opts.value.current &&
                this.opts.value.current.start &&
                this.opts.value.current.end) {
                this.opts.value.current.start = undefined;
                this.opts.value.current.end = undefined;
            }
        });
        this.shiftFocus = this.shiftFocus.bind(this);
        this.handleCellClick = this.handleCellClick.bind(this);
        this.onkeydown = this.onkeydown.bind(this);
        this.nextPage = this.nextPage.bind(this);
        this.prevPage = this.prevPage.bind(this);
        this.nextYear = this.nextYear.bind(this);
        this.prevYear = this.prevYear.bind(this);
        this.setYear = this.setYear.bind(this);
        this.setMonth = this.setMonth.bind(this);
        this.isDateDisabled = this.isDateDisabled.bind(this);
        this.isDateUnavailable = this.isDateUnavailable.bind(this);
        this.isOutsideVisibleMonths = this.isOutsideVisibleMonths.bind(this);
        this.isSelected = this.isSelected.bind(this);
        useEnsureNonDisabledPlaceholder({
            placeholder: opts.placeholder,
            defaultPlaceholder: opts.defaultPlaceholder,
            isDateDisabled: opts.isDateDisabled,
            maxValue: opts.maxValue,
            minValue: opts.minValue,
            ref: opts.ref,
        });
    }
    #updateValue(cb) {
        const value = this.opts.value.current;
        const newValue = cb(value);
        this.opts.value.current = newValue;
        if (newValue.start && newValue.end) {
            this.opts.onRangeSelect?.current?.();
        }
    }
    #setStartValue(value) {
        this.opts.startValue.current = value;
    }
    #setEndValue(value) {
        this.opts.endValue.current = value;
    }
    setMonths = (months) => {
        this.months = months;
    };
    /**
     * This derived state holds an array of localized day names for the current
     * locale and calendar view. It dynamically syncs with the 'weekStartsOn' option,
     * updating its content when the option changes. Using this state to render the
     * calendar's days of the week is strongly recommended, as it guarantees that
     * the days are correctly formatted for the current locale and calendar view.
     */
    weekdays = $derived.by(() => {
        return getWeekdays({
            months: this.months,
            formatter: this.formatter,
            weekdayFormat: this.opts.weekdayFormat.current,
        });
    });
    isOutsideVisibleMonths(date) {
        return !this.visibleMonths.some((month) => isSameMonth(date, month));
    }
    isDateDisabled(date) {
        if (this.opts.isDateDisabled.current(date) || this.opts.disabled.current)
            return true;
        const minValue = this.opts.minValue.current;
        const maxValue = this.opts.maxValue.current;
        if (minValue && isBefore(date, minValue))
            return true;
        if (maxValue && isAfter(date, maxValue))
            return true;
        return false;
    }
    isDateUnavailable(date) {
        if (this.opts.isDateUnavailable.current(date))
            return true;
        return false;
    }
    isStartInvalid = $derived.by(() => {
        if (!this.opts.startValue.current)
            return false;
        return (this.isDateUnavailable(this.opts.startValue.current) ||
            this.isDateDisabled(this.opts.startValue.current));
    });
    isEndInvalid = $derived.by(() => {
        if (!this.opts.endValue.current)
            return false;
        return (this.isDateUnavailable(this.opts.endValue.current) ||
            this.isDateDisabled(this.opts.endValue.current));
    });
    isInvalid = $derived.by(() => {
        if (this.isStartInvalid || this.isEndInvalid)
            return true;
        if (this.opts.endValue.current &&
            this.opts.startValue.current &&
            isBefore(this.opts.endValue.current, this.opts.startValue.current))
            return true;
        return false;
    });
    isNextButtonDisabled = $derived.by(() => {
        return getIsNextButtonDisabled({
            maxValue: this.opts.maxValue.current,
            months: this.months,
            disabled: this.opts.disabled.current,
        });
    });
    isPrevButtonDisabled = $derived.by(() => {
        return getIsPrevButtonDisabled({
            minValue: this.opts.minValue.current,
            months: this.months,
            disabled: this.opts.disabled.current,
        });
    });
    headingValue = $derived.by(() => {
        return getCalendarHeadingValue({
            months: this.months,
            formatter: this.formatter,
            locale: this.opts.locale.current,
        });
    });
    fullCalendarLabel = $derived.by(() => `${this.opts.calendarLabel.current} ${this.headingValue}`);
    isSelectionStart(date) {
        const ranges = this.opts.ranges.current ?? [];
        for (let i = 0; i < ranges.length; i++) {
            if (isSameDay(date, ranges[i].start)) {
                return true;
            }
        }
        if (!this.opts.startValue.current)
            return false;
        return isSameDay(date, this.opts.startValue.current);
    }
    isSelectionEnd(date) {
        const ranges = this.opts.ranges.current ?? [];
        for (let i = 0; i < ranges.length; i++) {
            if (isSameDay(date, ranges[i].end)) {
                return true;
            }
        }
        if (!this.opts.endValue.current)
            return false;
        return isSameDay(date, this.opts.endValue.current);
    }
    isSelected(date) {
        const ranges = this.opts.ranges.current ?? [];
        for (let i = 0; i < ranges.length; i++) {
            if (isBetweenInclusive(date, ranges[i].start, ranges[i].end)) {
                return true;
            }
        }
        if (this.opts.startValue.current && isSameDay(this.opts.startValue.current, date))
            return true;
        if (this.opts.endValue.current && isSameDay(this.opts.endValue.current, date))
            return true;
        if (this.opts.startValue.current && this.opts.endValue.current) {
            return isBetweenInclusive(date, this.opts.startValue.current, this.opts.endValue.current);
        }
        return false;
    }
    highlightedRange = $derived.by(() => {
        if (this.opts.startValue.current && this.opts.endValue.current)
            return null;
        if (!this.opts.startValue.current || !this.focusedValue)
            return null;
        const isStartBeforeFocused = isBefore(this.opts.startValue.current, this.focusedValue);
        const start = isStartBeforeFocused ? this.opts.startValue.current : this.focusedValue;
        const end = isStartBeforeFocused ? this.focusedValue : this.opts.startValue.current;
        const range = { start, end };
        if (isSameDay(start.add({ days: 1 }), end) || isSameDay(start, end)) {
            return range;
        }
        const isValid = areAllDaysBetweenValid(start, end, this.isDateUnavailable, this.isDateDisabled);
        if (isValid)
            return range;
        return null;
    });
    shiftFocus(node, add) {
        return shiftCalendarFocus({
            node,
            add,
            placeholder: this.opts.placeholder,
            calendarNode: this.opts.ref.current,
            isPrevButtonDisabled: this.isPrevButtonDisabled,
            isNextButtonDisabled: this.isNextButtonDisabled,
            months: this.months,
            numberOfMonths: this.opts.numberOfMonths.current,
        });
    }
    #announceEmpty() {
        this.announcer.announce("Selected date is now empty.", "polite");
    }
    #announceSelectedDate(date) {
        this.announcer.announce(`Selected Date: ${this.formatter.selectedDate(date, false)}`, "polite");
    }
    #announceSelectedRange(start, end) {
        this.announcer.announce(`Selected Dates: ${this.formatter.selectedDate(start, false)} to ${this.formatter.selectedDate(end, false)}`, "polite");
    }
    handleCellClick(e, date) {
        if (this.isDateDisabled(date) || this.isDateUnavailable(date))
            return;

        const ranges = this.opts.ranges.current ?? [];

        if (this.opts.learnMore.current) {
            for (let i = 0; i < ranges.length; i++) {
                if (isBetweenInclusive(date, ranges[i].start, ranges[i].end)) {
                    this.opts.learnMore.current(date, i);
                    return;
                }
            }
        }
        const otherSelection = this.opts.startValue.current ?? this.opts.endValue.current ?? undefined;
        if (otherSelection) {
            for (let i = 0; i < ranges.length; i++) {
                if (isBefore(otherSelection, ranges[i].start) || isSameDay(otherSelection, ranges[i].start)) {
                    if (isAfter(date, ranges[i].start)) {
                        this.#announceSelectedDate(date);
                        this.#setStartValue(date);
                        this.#setEndValue(undefined);
                        return;
                    }
                }
                if (isAfter(otherSelection, ranges[i].end) || isSameDay(otherSelection, ranges[i].end)) {
                    if (isBefore(date, ranges[i].end)) {
                        this.#announceSelectedDate(date);
                        this.#setStartValue(date);
                        this.#setEndValue(undefined);
                        return;
                    }
                }
            }
        }
        const prevLastPressedDate = this.lastPressedDateValue;
        this.lastPressedDateValue = date;
        if (this.opts.startValue.current && this.highlightedRange === null) {
            if (isSameDay(this.opts.startValue.current, date) &&
                !this.opts.preventDeselect.current &&
                !this.opts.endValue.current) {
                this.#setStartValue(undefined);
                this.opts.placeholder.current = date;
                this.#announceEmpty();
                return;
            }
            else if (!this.opts.endValue.current) {
                e.preventDefault();
                if (prevLastPressedDate && isSameDay(prevLastPressedDate, date)) {
                    this.#setStartValue(date);
                    this.#announceSelectedDate(date);
                }
            }
        }
        if (this.opts.startValue.current && this.opts.endValue.current &&
            (isSameDay(this.opts.startValue.current, date) || isSameDay(this.opts.endValue.current, date)) &&
            !this.opts.preventDeselect.current) {
            this.#setStartValue(undefined);
            this.#setEndValue(undefined);
            this.opts.placeholder.current = date;
            this.#announceEmpty();
            return;
        }
        if (!this.opts.startValue.current) {
            this.#announceSelectedDate(date);
            this.#setStartValue(date);
        }
        else if (!this.opts.endValue.current) {
            this.#announceSelectedRange(this.opts.startValue.current, date);
            this.#setEndValue(date);
        }
        else if (this.opts.endValue.current && this.opts.startValue.current) {
            this.#setEndValue(undefined);
            this.#announceSelectedDate(date);
            this.#setStartValue(date);
        }
    }
    onkeydown(event) {
        return handleCalendarKeydown({
            event,
            handleCellClick: this.handleCellClick,
            placeholderValue: this.opts.placeholder.current,
            shiftFocus: this.shiftFocus,
        });
    }
    /**
     * Navigates to the next page of the calendar.
     */
    nextPage() {
        handleCalendarNextPage({
            fixedWeeks: this.opts.fixedWeeks.current,
            locale: this.opts.locale.current,
            numberOfMonths: this.opts.numberOfMonths.current,
            pagedNavigation: this.opts.pagedNavigation.current,
            setMonths: this.setMonths,
            setPlaceholder: (date) => (this.opts.placeholder.current = date),
            weekStartsOn: this.opts.weekStartsOn.current,
            months: this.months,
        });
    }
    /**
     * Navigates to the previous page of the calendar.
     */
    prevPage() {
        handleCalendarPrevPage({
            fixedWeeks: this.opts.fixedWeeks.current,
            locale: this.opts.locale.current,
            numberOfMonths: this.opts.numberOfMonths.current,
            pagedNavigation: this.opts.pagedNavigation.current,
            setMonths: this.setMonths,
            setPlaceholder: (date) => (this.opts.placeholder.current = date),
            weekStartsOn: this.opts.weekStartsOn.current,
            months: this.months,
        });
    }
    nextYear() {
        this.opts.placeholder.current = this.opts.placeholder.current.add({ years: 1 });
    }
    prevYear() {
        this.opts.placeholder.current = this.opts.placeholder.current.subtract({ years: 1 });
    }
    setYear(year) {
        this.opts.placeholder.current = this.opts.placeholder.current.set({ year });
    }
    setMonth(month) {
        this.opts.placeholder.current = this.opts.placeholder.current.set({ month });
    }
    getBitsAttr(part) {
        return `data-range-calendar-${part}`;
    }
    snippetProps = $derived.by(() => ({
        months: this.months,
        weekdays: this.weekdays,
    }));
    props = $derived.by(() => ({
        ...getCalendarElementProps({
            fullCalendarLabel: this.fullCalendarLabel,
            id: this.opts.id.current,
            isInvalid: this.isInvalid,
            disabled: this.opts.disabled.current,
            readonly: this.opts.readonly.current,
        }),
        [this.getBitsAttr("root")]: "",
        //
        onkeydown: this.onkeydown,
    }));
}
export class RangeCalendarCellState {
    opts;
    root;
    cellDate = $derived.by(() => toDate(this.opts.date.current));
    isDisabled = $derived.by(() => this.root.isDateDisabled(this.opts.date.current));
    isUnavailable = $derived.by(() => this.root.opts.isDateUnavailable.current(this.opts.date.current));
    isDateToday = $derived.by(() => isToday(this.opts.date.current, getLocalTimeZone()));
    isOutsideMonth = $derived.by(() => !isSameMonth(this.opts.date.current, this.opts.month.current));
    isOutsideVisibleMonths = $derived.by(() => this.root.isOutsideVisibleMonths(this.opts.date.current));
    isFocusedDate = $derived.by(() => isSameDay(this.opts.date.current, this.root.opts.placeholder.current));
    isSelectedDate = $derived.by(() => this.root.isSelected(this.opts.date.current));
    isSelectionStart = $derived.by(() => this.root.isSelectionStart(this.opts.date.current));
    isSelectionEnd = $derived.by(() => this.root.isSelectionEnd(this.opts.date.current));
    isHighlighted = $derived.by(() => this.root.highlightedRange
        ? isBetweenInclusive(this.opts.date.current, this.root.highlightedRange.start, this.root.highlightedRange.end)
        : false);
    labelText = $derived.by(() => this.root.formatter.custom(this.cellDate, {
        weekday: "long",
        month: "long",
        day: "numeric",
        year: "numeric",
    }));
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        useRefById(opts);
    }
    snippetProps = $derived.by(() => ({
        disabled: this.isDisabled,
        unavailable: this.isUnavailable,
        selected: this.isSelectedDate,
    }));
    ariaDisabled = $derived.by(() => {
        return (this.isDisabled ||
            (this.isOutsideMonth && this.root.opts.disableDaysOutsideMonth.current) ||
            this.isUnavailable);
    });
    sharedDataAttrs = $derived.by(() => ({
        "data-unavailable": getDataUnavailable(this.isUnavailable),
        "data-today": this.isDateToday ? "" : undefined,
        "data-outside-month": this.isOutsideMonth ? "" : undefined,
        "data-outside-visible-months": this.isOutsideVisibleMonths ? "" : undefined,
        "data-focused": this.isFocusedDate ? "" : undefined,
        "data-selection-start": this.isSelectionStart ? "" : undefined,
        "data-selection-end": this.isSelectionEnd ? "" : undefined,
        "data-highlighted": this.isHighlighted ? "" : undefined,
        "data-selected": getDataSelected(this.isSelectedDate),
        "data-value": this.opts.date.current.toString(),
        "data-type": getDateValueType(this.opts.date.current),
        "data-disabled": getDataDisabled(this.isDisabled ||
            (this.isOutsideMonth && this.root.opts.disableDaysOutsideMonth.current)),
    }));
    props = $derived.by(() => ({
        id: this.opts.id.current,
        role: "gridcell",
        "aria-selected": getAriaSelected(this.isSelectedDate),
        "aria-disabled": getAriaDisabled(this.ariaDisabled),
        ...this.sharedDataAttrs,
        [this.root.getBitsAttr("cell")]: "",
    }));
}
class RangeCalendarDayState {
    opts;
    cell;
    constructor(opts, cell) {
        this.opts = opts;
        this.cell = cell;
        useRefById(opts);
        this.onclick = this.onclick.bind(this);
        this.onmouseenter = this.onmouseenter.bind(this);
        this.onfocusin = this.onfocusin.bind(this);
    }
    #tabindex = $derived.by(() => (this.cell.isOutsideMonth && this.cell.root.opts.disableDaysOutsideMonth.current) ||
        this.cell.isDisabled
        ? undefined
        : this.cell.isFocusedDate
            ? 0
            : -1);
    onclick(e) {
        if (this.cell.isDisabled)
            return;
        this.cell.root.handleCellClick(e, this.cell.opts.date.current);
    }
    onmouseenter(_) {
        if (this.cell.isDisabled)
            return;
        this.cell.root.focusedValue = this.cell.opts.date.current;
    }
    onfocusin(_) {
        if (this.cell.isDisabled)
            return;
        this.cell.root.focusedValue = this.cell.opts.date.current;
    }
    snippetProps = $derived.by(() => ({
        disabled: this.cell.isDisabled,
        unavailable: this.cell.isUnavailable,
        selected: this.cell.isSelectedDate,
        day: `${this.cell.opts.date.current.day}`,
    }));
    props = $derived.by(() => ({
        id: this.opts.id.current,
        role: "button",
        "aria-label": this.cell.labelText,
        "aria-disabled": getAriaDisabled(this.cell.ariaDisabled),
        ...this.cell.sharedDataAttrs,
        tabindex: this.#tabindex,
        [this.cell.root.getBitsAttr("day")]: "",
        // Shared logic for range calendar and calendar
        "data-bits-day": "",
        //
        onclick: this.onclick,
        onmouseenter: this.onmouseenter,
        onfocusin: this.onfocusin,
    }));
}
const RangeCalendarCellContext = new Context("RangeCalendar.Cell");
export function useRangeCalendarRoot(props) {
    return CalendarRootContext.set(new RangeCalendarRootState(props));
}
export function useRangeCalendarCell(props) {
    return RangeCalendarCellContext.set(new RangeCalendarCellState(props, CalendarRootContext.get()));
}
export function useRangeCalendarDay(props) {
    return new RangeCalendarDayState(props, RangeCalendarCellContext.get());
}
