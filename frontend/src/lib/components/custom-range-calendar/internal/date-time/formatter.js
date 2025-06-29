import { DateFormatter } from "@internationalized/date";
import { hasTime, isZonedDateTime, toDate } from "./utils.js";
const defaultPartOptions = {
    year: "numeric",
    month: "numeric",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
    second: "numeric",
};
/**
 * Creates a wrapper around the `DateFormatter`, which is
 * an improved version of the {@link Intl.DateTimeFormat} API,
 * that is used internally by the various date builders to
 * easily format dates in a consistent way.
 *
 * @see [DateFormatter](https://react-spectrum.adobe.com/internationalized/date/DateFormatter.html)
 */
export function createFormatter(initialLocale) {
    let locale = initialLocale;
    function setLocale(newLocale) {
        locale = newLocale;
    }
    function getLocale() {
        return locale;
    }
    function custom(date, options) {
        return new DateFormatter(locale, options).format(date);
    }
    function selectedDate(date, includeTime = true) {
        if (hasTime(date) && includeTime) {
            return custom(toDate(date), {
                dateStyle: "long",
                timeStyle: "long",
            });
        }
        else {
            return custom(toDate(date), {
                dateStyle: "long",
            });
        }
    }
    function fullMonthAndYear(date) {
        return new DateFormatter(locale, { month: "long", year: "numeric" }).format(date);
    }
    function fullMonth(date) {
        return new DateFormatter(locale, { month: "long" }).format(date);
    }
    function fullYear(date) {
        return new DateFormatter(locale, { year: "numeric" }).format(date);
    }
    function toParts(date, options) {
        if (isZonedDateTime(date)) {
            return new DateFormatter(locale, {
                ...options,
                timeZone: date.timeZone,
            }).formatToParts(toDate(date));
        }
        else {
            return new DateFormatter(locale, options).formatToParts(toDate(date));
        }
    }
    function dayOfWeek(date, length = "narrow") {
        return new DateFormatter(locale, { weekday: length }).format(date);
    }
    function dayPeriod(date, hourCycle = undefined) {
        const parts = new DateFormatter(locale, {
            hour: "numeric",
            minute: "numeric",
            hourCycle: hourCycle === 24 ? "h23" : undefined,
        }).formatToParts(date);
        const value = parts.find((p) => p.type === "dayPeriod")?.value;
        if (value === "PM") {
            return "PM";
        }
        return "AM";
    }
    function part(dateObj, type, options = {}) {
        const opts = { ...defaultPartOptions, ...options };
        const parts = toParts(dateObj, opts);
        const part = parts.find((p) => p.type === type);
        return part ? part.value : "";
    }
    return {
        setLocale,
        getLocale,
        fullMonth,
        fullYear,
        fullMonthAndYear,
        toParts,
        custom,
        part,
        dayPeriod,
        selectedDate,
        dayOfWeek,
    };
}
