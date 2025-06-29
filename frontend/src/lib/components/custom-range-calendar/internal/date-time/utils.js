import { CalendarDate, CalendarDateTime, ZonedDateTime, getDayOfWeek, getLocalTimeZone, parseDate, parseDateTime, parseZonedDateTime, toCalendar, } from "@internationalized/date";
const defaultDateDefaults = {
    defaultValue: undefined,
    granularity: "day",
};
/**
 * A helper function used throughout the various date builders
 * to generate a default `DateValue` using the `defaultValue`,
 * `defaultPlaceholder`, and `granularity` props.
 *
 * It's important to match the `DateValue` type being used
 * elsewhere in the builder, so they behave according to the
 * behavior the user expects based on the props they've provided.
 *
 */
export function getDefaultDate(opts) {
    const withDefaults = { ...defaultDateDefaults, ...opts };
    const { defaultValue, granularity } = withDefaults;
    if (Array.isArray(defaultValue) && defaultValue.length) {
        return defaultValue[defaultValue.length - 1];
    }
    if (defaultValue && !Array.isArray(defaultValue)) {
        return defaultValue;
    }
    else {
        const date = new Date();
        const year = date.getFullYear();
        const month = date.getMonth() + 1;
        const day = date.getDate();
        const calendarDateTimeGranularities = ["hour", "minute", "second"];
        if (calendarDateTimeGranularities.includes(granularity ?? "day")) {
            return new CalendarDateTime(year, month, day, 0, 0, 0);
        }
        return new CalendarDate(year, month, day);
    }
}
/**
 * Given a date string and a reference `DateValue` object, parse the
 * string to the same type as the reference object.
 *
 * Useful for parsing strings from data attributes, which are always
 * strings, to the same type being used by the date component.
 */
export function parseStringToDateValue(dateStr, referenceVal) {
    let dateValue;
    if (referenceVal instanceof ZonedDateTime) {
        dateValue = parseZonedDateTime(dateStr);
    }
    else if (referenceVal instanceof CalendarDateTime) {
        dateValue = parseDateTime(dateStr);
    }
    else {
        dateValue = parseDate(dateStr);
    }
    // ensure the parsed date is in the same calendar as the reference date set by the user.
    return dateValue.calendar !== referenceVal.calendar
        ? toCalendar(dateValue, referenceVal.calendar)
        : dateValue;
}
/**
 * Given a `DateValue` object, convert it to a native `Date` object.
 * If a timezone is provided, the date will be converted to that timezone.
 * If no timezone is provided, the date will be converted to the local timezone.
 */
export function toDate(dateValue, tz = getLocalTimeZone()) {
    if (dateValue instanceof ZonedDateTime) {
        return dateValue.toDate();
    }
    else {
        return dateValue.toDate(tz);
    }
}
export function getDateValueType(date) {
    if (date instanceof CalendarDate)
        return "date";
    if (date instanceof CalendarDateTime)
        return "datetime";
    if (date instanceof ZonedDateTime)
        return "zoneddatetime";
    throw new Error("Unknown date type");
}
export function parseAnyDateValue(value, type) {
    switch (type) {
        case "date":
            return parseDate(value);
        case "datetime":
            return parseDateTime(value);
        case "zoneddatetime":
            return parseZonedDateTime(value);
        default:
            throw new Error(`Unknown date type: ${type}`);
    }
}
function isCalendarDateTime(dateValue) {
    return dateValue instanceof CalendarDateTime;
}
export function isZonedDateTime(dateValue) {
    return dateValue instanceof ZonedDateTime;
}
export function hasTime(dateValue) {
    return isCalendarDateTime(dateValue) || isZonedDateTime(dateValue);
}
/**
 * Given a date, return the number of days in the month.
 */
export function getDaysInMonth(date) {
    if (date instanceof Date) {
        const year = date.getFullYear();
        const month = date.getMonth() + 1;
        /**
         * By using zero as the day, we get the
         * last day of the previous month, which
         * is the month we originally passed in.
         */
        return new Date(year, month, 0).getDate();
    }
    else {
        return date.set({ day: 100 }).day;
    }
}
/**
 * Determine if a date is before the reference date.
 * @param dateToCompare - is this date before the `referenceDate`
 * @param referenceDate - is the `dateToCompare` before this date
 *
 * @see {@link isBeforeOrSame} for inclusive
 */
export function isBefore(dateToCompare, referenceDate) {
    return dateToCompare.compare(referenceDate) < 0;
}
/**
 * Determine if a date is after the reference date.
 * @param dateToCompare - is this date after the `referenceDate`
 * @param referenceDate - is the `dateToCompare` after this date
 *
 * @see {@link isAfterOrSame} for inclusive
 */
export function isAfter(dateToCompare, referenceDate) {
    return dateToCompare.compare(referenceDate) > 0;
}
/**
 * Determine if a date is before or the same as the reference date.
 *
 * @param dateToCompare - the date to compare
 * @param referenceDate - the reference date to make the comparison against
 *
 * @see {@link isBefore} for non-inclusive
 */
function isBeforeOrSame(dateToCompare, referenceDate) {
    return dateToCompare.compare(referenceDate) <= 0;
}
/**
 * Determine if a date is after or the same as the reference date.
 *
 * @param dateToCompare - is this date after or the same as the `referenceDate`
 * @param referenceDate - is the `dateToCompare` after or the same as this date
 *
 * @see {@link isAfter} for non-inclusive
 */
function isAfterOrSame(dateToCompare, referenceDate) {
    return dateToCompare.compare(referenceDate) >= 0;
}
/**
 * Determine if a date is inclusively between a start and end reference date.
 *
 * @param date - is this date inclusively between the `start` and `end` dates
 * @param start - the start reference date to make the comparison against
 * @param end - the end reference date to make the comparison against
 *
 * @see {@link isBetween} for non-inclusive
 */
export function isBetweenInclusive(date, start, end) {
    return isAfterOrSame(date, start) && isBeforeOrSame(date, end);
}
export function getLastFirstDayOfWeek(date, firstDayOfWeek, locale) {
    const day = getDayOfWeek(date, locale);
    if (firstDayOfWeek > day) {
        return date.subtract({ days: day + 7 - firstDayOfWeek });
    }
    if (firstDayOfWeek === day) {
        return date;
    }
    return date.subtract({ days: day - firstDayOfWeek });
}
export function getNextLastDayOfWeek(date, firstDayOfWeek, locale) {
    const day = getDayOfWeek(date, locale);
    const lastDayOfWeek = firstDayOfWeek === 0 ? 6 : firstDayOfWeek - 1;
    if (day === lastDayOfWeek) {
        return date;
    }
    if (day > lastDayOfWeek) {
        return date.add({ days: 7 - day + lastDayOfWeek });
    }
    return date.add({ days: lastDayOfWeek - day });
}
export function areAllDaysBetweenValid(start, end, isUnavailable, isDisabled) {
    if (isUnavailable === undefined && isDisabled === undefined) {
        return true;
    }
    let dCurrent = start.add({ days: 1 });
    if (isDisabled?.(dCurrent) || isUnavailable?.(dCurrent)) {
        return false;
    }
    const dEnd = end;
    while (dCurrent.compare(dEnd) < 0) {
        dCurrent = dCurrent.add({ days: 1 });
        if (isDisabled?.(dCurrent) || isUnavailable?.(dCurrent)) {
            return false;
        }
    }
    return true;
}
