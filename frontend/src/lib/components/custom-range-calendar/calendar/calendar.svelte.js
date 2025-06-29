import { getLocalTimeZone, isSameDay, isSameMonth, isToday, } from "@internationalized/date";
import { DEV } from "esm-env";
import { untrack } from "svelte";
import { useRefById } from "svelte-toolbelt";
import { Context, watch } from "runed";
import { getAriaDisabled, getAriaHidden, getAriaReadonly, getAriaSelected, getDataDisabled, getDataReadonly, getDataSelected, getDataUnavailable, } from "../internal/attrs.js";
import { useId } from "bits-ui";
import { getAnnouncer } from "../internal/date-time/announcer.js";
import { createFormatter } from "../internal/date-time/formatter.js";
import { createAccessibleHeading, createMonths, getCalendarElementProps, getCalendarHeadingValue, getDateWithPreviousTime, getIsNextButtonDisabled, getIsPrevButtonDisabled, getWeekdays, handleCalendarKeydown, handleCalendarNextPage, handleCalendarPrevPage, shiftCalendarFocus, useEnsureNonDisabledPlaceholder, useMonthViewOptionsSync, useMonthViewPlaceholderSync, } from "../internal/date-time/calendar-helpers.svelte.js";
import { getDateValueType, isBefore, toDate } from "../internal/date-time/utils.js";
export class CalendarRootState {
    opts;
    months = $state([]);
    visibleMonths = $derived.by(() => this.months.map((month) => month.value));
    announcer;
    formatter;
    accessibleHeadingId = useId();
    constructor(opts) {
        this.opts = opts;
        this.announcer = getAnnouncer();
        this.formatter = createFormatter(this.opts.locale.current);
        this.setMonths = this.setMonths.bind(this);
        this.nextPage = this.nextPage.bind(this);
        this.prevPage = this.prevPage.bind(this);
        this.prevYear = this.prevYear.bind(this);
        this.nextYear = this.nextYear.bind(this);
        this.setYear = this.setYear.bind(this);
        this.setMonth = this.setMonth.bind(this);
        this.isOutsideVisibleMonths = this.isOutsideVisibleMonths.bind(this);
        this.isDateDisabled = this.isDateDisabled.bind(this);
        this.isDateSelected = this.isDateSelected.bind(this);
        this.shiftFocus = this.shiftFocus.bind(this);
        this.handleCellClick = this.handleCellClick.bind(this);
        this.handleMultipleUpdate = this.handleMultipleUpdate.bind(this);
        this.handleSingleUpdate = this.handleSingleUpdate.bind(this);
        this.onkeydown = this.onkeydown.bind(this);
        this.getBitsAttr = this.getBitsAttr.bind(this);
        useRefById(opts);
        this.months = createMonths({
            dateObj: this.opts.placeholder.current,
            weekStartsOn: this.opts.weekStartsOn.current,
            locale: this.opts.locale.current,
            fixedWeeks: this.opts.fixedWeeks.current,
            numberOfMonths: this.opts.numberOfMonths.current,
        });
        $effect(() => {
            const initialFocus = untrack(() => this.opts.initialFocus.current);
            if (initialFocus) {
                // focus the first `data-focused` day node
                const firstFocusedDay = this.opts.ref.current?.querySelector(`[data-focused]`);
                if (firstFocusedDay) {
                    firstFocusedDay.focus();
                }
            }
        });
        $effect(() => {
            if (!this.opts.ref.current)
                return;
            const removeHeading = createAccessibleHeading({
                calendarNode: this.opts.ref.current,
                label: this.fullCalendarLabel,
                accessibleHeadingId: this.accessibleHeadingId,
            });
            return removeHeading;
        });
        $effect(() => {
            if (this.formatter.getLocale() === this.opts.locale.current)
                return;
            this.formatter.setLocale(this.opts.locale.current);
        });
        /**
         * Updates the displayed months based on changes in the placeholder value.
         */
        useMonthViewPlaceholderSync({
            placeholder: this.opts.placeholder,
            getVisibleMonths: () => this.visibleMonths,
            weekStartsOn: this.opts.weekStartsOn,
            locale: this.opts.locale,
            fixedWeeks: this.opts.fixedWeeks,
            numberOfMonths: this.opts.numberOfMonths,
            setMonths: (months) => (this.months = months),
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
         * Synchronize the placeholder value with the current value.
         */
        watch(() => this.opts.value.current, () => {
            const value = this.opts.value.current;
            if (Array.isArray(value) && value.length) {
                const lastValue = value[value.length - 1];
                if (lastValue && this.opts.placeholder.current !== lastValue) {
                    this.opts.placeholder.current = lastValue;
                }
            }
            else if (!Array.isArray(value) &&
                value &&
                this.opts.placeholder.current !== value) {
                this.opts.placeholder.current = value;
            }
        });
        useEnsureNonDisabledPlaceholder({
            placeholder: opts.placeholder,
            defaultPlaceholder: opts.defaultPlaceholder,
            isDateDisabled: opts.isDateDisabled,
            maxValue: opts.maxValue,
            minValue: opts.minValue,
            ref: opts.ref,
        });
    }
    setMonths(months) {
        this.months = months;
    }
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
    isInvalid = $derived.by(() => {
        const value = this.opts.value.current;
        const isDateDisabled = this.opts.isDateDisabled.current;
        const isDateUnavailable = this.opts.isDateUnavailable.current;
        if (Array.isArray(value)) {
            if (!value.length)
                return false;
            for (const date of value) {
                if (isDateDisabled(date))
                    return true;
                if (isDateUnavailable(date))
                    return true;
            }
        }
        else {
            if (!value)
                return false;
            if (isDateDisabled(value))
                return true;
            if (isDateUnavailable(value))
                return true;
        }
        return false;
    });
    headingValue = $derived.by(() => {
        return getCalendarHeadingValue({
            months: this.months,
            formatter: this.formatter,
            locale: this.opts.locale.current,
        });
    });
    fullCalendarLabel = $derived.by(() => {
        return `${this.opts.calendarLabel.current} ${this.headingValue}`;
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
        if (maxValue && isBefore(maxValue, date))
            return true;
        return false;
    }
    isDateSelected(date) {
        const value = this.opts.value.current;
        if (Array.isArray(value)) {
            return value.some((d) => isSameDay(d, date));
        }
        else if (!value) {
            return false;
        }
        return isSameDay(value, date);
    }
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
    handleCellClick(_, date) {
        if (this.opts.readonly.current)
            return;
        if (this.opts.isDateDisabled.current?.(date) ||
            this.opts.isDateUnavailable.current?.(date)) {
            return;
        }
        const prev = this.opts.value.current;
        const multiple = this.opts.type.current === "multiple";
        if (multiple) {
            if (Array.isArray(prev) || prev === undefined) {
                this.opts.value.current = this.handleMultipleUpdate(prev, date);
            }
        }
        else if (!Array.isArray(prev)) {
            const next = this.handleSingleUpdate(prev, date);
            if (!next) {
                this.announcer.announce("Selected date is now empty.", "polite", 5000);
            }
            else {
                this.announcer.announce(`Selected Date: ${this.formatter.selectedDate(next, false)}`, "polite");
            }
            this.opts.value.current = getDateWithPreviousTime(next, prev);
            if (next !== undefined) {
                this.opts.onDateSelect?.current?.();
            }
        }
    }
    handleMultipleUpdate(prev, date) {
        if (!prev)
            return [date];
        if (!Array.isArray(prev)) {
            if (DEV)
                throw new Error("Invalid value for multiple prop.");
            return;
        }
        const index = prev.findIndex((d) => isSameDay(d, date));
        const preventDeselect = this.opts.preventDeselect.current;
        if (index === -1) {
            return [...prev, date];
        }
        else if (preventDeselect) {
            return prev;
        }
        else {
            const next = prev.filter((d) => !isSameDay(d, date));
            if (!next.length) {
                this.opts.placeholder.current = date;
                return undefined;
            }
            return next;
        }
    }
    handleSingleUpdate(prev, date) {
        if (Array.isArray(prev)) {
            if (DEV)
                throw new Error("Invalid value for single prop.");
        }
        if (!prev)
            return date;
        const preventDeselect = this.opts.preventDeselect.current;
        if (!preventDeselect && isSameDay(prev, date)) {
            this.opts.placeholder.current = date;
            return undefined;
        }
        return date;
    }
    onkeydown(event) {
        handleCalendarKeydown({
            event,
            handleCellClick: this.handleCellClick,
            shiftFocus: this.shiftFocus,
            placeholderValue: this.opts.placeholder.current,
        });
    }
    snippetProps = $derived.by(() => ({
        months: this.months,
        weekdays: this.weekdays,
    }));
    getBitsAttr(part) {
        return `data-bits-calendar-${part}`;
    }
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
export class CalendarHeadingState {
    opts;
    root;
    headingValue = $derived.by(() => this.root.headingValue);
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        useRefById(opts);
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        "aria-hidden": getAriaHidden(true),
        "data-disabled": getDataDisabled(this.root.opts.disabled.current),
        "data-readonly": getDataReadonly(this.root.opts.readonly.current),
        [this.root.getBitsAttr("heading")]: "",
    }));
}
class CalendarCellState {
    opts;
    root;
    cellDate = $derived.by(() => toDate(this.opts.date.current));
    isDisabled = $derived.by(() => this.root.isDateDisabled(this.opts.date.current));
    isUnavailable = $derived.by(() => this.root.opts.isDateUnavailable.current(this.opts.date.current));
    isDateToday = $derived.by(() => isToday(this.opts.date.current, getLocalTimeZone()));
    isOutsideMonth = $derived.by(() => !isSameMonth(this.opts.date.current, this.opts.month.current));
    isOutsideVisibleMonths = $derived.by(() => this.root.isOutsideVisibleMonths(this.opts.date.current));
    isFocusedDate = $derived.by(() => isSameDay(this.opts.date.current, this.root.opts.placeholder.current));
    isSelectedDate = $derived.by(() => this.root.isDateSelected(this.opts.date.current));
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
class CalendarDayState {
    opts;
    cell;
    constructor(opts, cell) {
        this.opts = opts;
        this.cell = cell;
        this.onclick = this.onclick.bind(this);
        useRefById(opts);
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
    }));
}
export class CalendarNextButtonState {
    opts;
    root;
    isDisabled = $derived.by(() => this.root.isNextButtonDisabled);
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        this.onclick = this.onclick.bind(this);
        useRefById(opts);
    }
    onclick(_) {
        if (this.isDisabled)
            return;
        this.root.nextPage();
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        role: "button",
        type: "button",
        "aria-label": "Next",
        "aria-disabled": getAriaDisabled(this.isDisabled),
        "data-disabled": getDataDisabled(this.isDisabled),
        disabled: this.isDisabled,
        [this.root.getBitsAttr("next-button")]: "",
        //
        onclick: this.onclick,
    }));
}
export class CalendarPrevButtonState {
    opts;
    root;
    isDisabled = $derived.by(() => this.root.isPrevButtonDisabled);
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        this.onclick = this.onclick.bind(this);
        useRefById(opts);
    }
    onclick(_) {
        if (this.isDisabled)
            return;
        this.root.prevPage();
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        role: "button",
        type: "button",
        "aria-label": "Previous",
        "aria-disabled": getAriaDisabled(this.isDisabled),
        "data-disabled": getDataDisabled(this.isDisabled),
        disabled: this.isDisabled,
        [this.root.getBitsAttr("prev-button")]: "",
        //
        onclick: this.onclick,
    }));
}
export class CalendarGridState {
    opts;
    root;
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        useRefById(opts);
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        tabindex: -1,
        role: "grid",
        "aria-readonly": getAriaReadonly(this.root.opts.readonly.current),
        "aria-disabled": getAriaDisabled(this.root.opts.disabled.current),
        "data-readonly": getDataReadonly(this.root.opts.readonly.current),
        "data-disabled": getDataDisabled(this.root.opts.disabled.current),
        [this.root.getBitsAttr("grid")]: "",
    }));
}
export class CalendarGridBodyState {
    opts;
    root;
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        useRefById(opts);
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        "data-disabled": getDataDisabled(this.root.opts.disabled.current),
        "data-readonly": getDataReadonly(this.root.opts.readonly.current),
        [this.root.getBitsAttr("grid-body")]: "",
    }));
}
export class CalendarGridHeadState {
    opts;
    root;
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        useRefById(opts);
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        "data-disabled": getDataDisabled(this.root.opts.disabled.current),
        "data-readonly": getDataReadonly(this.root.opts.readonly.current),
        [this.root.getBitsAttr("grid-head")]: "",
    }));
}
export class CalendarGridRowState {
    opts;
    root;
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        useRefById(opts);
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        "data-disabled": getDataDisabled(this.root.opts.disabled.current),
        "data-readonly": getDataReadonly(this.root.opts.readonly.current),
        [this.root.getBitsAttr("grid-row")]: "",
    }));
}
export class CalendarHeadCellState {
    opts;
    root;
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        useRefById(opts);
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        "data-disabled": getDataDisabled(this.root.opts.disabled.current),
        "data-readonly": getDataReadonly(this.root.opts.readonly.current),
        [this.root.getBitsAttr("head-cell")]: "",
    }));
}
export class CalendarHeaderState {
    opts;
    root;
    constructor(opts, root) {
        this.opts = opts;
        this.root = root;
        useRefById(opts);
    }
    props = $derived.by(() => ({
        id: this.opts.id.current,
        "data-disabled": getDataDisabled(this.root.opts.disabled.current),
        "data-readonly": getDataReadonly(this.root.opts.readonly.current),
        [this.root.getBitsAttr("header")]: "",
    }));
}
export const CalendarRootContext = new Context("Calendar.Root | RangeCalender.Root");
const CalendarCellContext = new Context("Calendar.Cell | RangeCalendar.Cell");
export function useCalendarRoot(props) {
    return CalendarRootContext.set(new CalendarRootState(props));
}
export function useCalendarGrid(props) {
    return new CalendarGridState(props, CalendarRootContext.get());
}
export function useCalendarCell(props) {
    return CalendarCellContext.set(new CalendarCellState(props, CalendarRootContext.get()));
}
export function useCalendarNextButton(props) {
    return new CalendarNextButtonState(props, CalendarRootContext.get());
}
export function useCalendarPrevButton(props) {
    return new CalendarPrevButtonState(props, CalendarRootContext.get());
}
export function useCalendarDay(props) {
    return new CalendarDayState(props, CalendarCellContext.get());
}
export function useCalendarGridBody(props) {
    return new CalendarGridBodyState(props, CalendarRootContext.get());
}
export function useCalendarGridHead(props) {
    return new CalendarGridHeadState(props, CalendarRootContext.get());
}
export function useCalendarGridRow(props) {
    return new CalendarGridRowState(props, CalendarRootContext.get());
}
export function useCalendarHeadCell(props) {
    return new CalendarHeadCellState(props, CalendarRootContext.get());
}
export function useCalendarHeader(props) {
    return new CalendarHeaderState(props, CalendarRootContext.get());
}
export function useCalendarHeading(props) {
    return new CalendarHeadingState(props, CalendarRootContext.get());
}
