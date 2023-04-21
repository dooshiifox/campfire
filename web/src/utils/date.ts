import { i18n } from "$/i18n";

/** For use with the `i18n` function.
  * 
  * # Example
  * 
  * ```ts
  * // If `DATE_FORMAT` is "Today at {HH}:{MM} {AMPM}", then
  * i18n('DATE_FORMAT', i18nDateFormats(new Date()));
  * // will return something like "Today at 12:34 PM"
  * ``` 
  */
export function i18nDateFormats(date: Date) {
    return {
        yyyy: date.getFullYear().toString(),
        yy: date.getFullYear().toString().slice(2),
        mm: (() => {
            const m = date.getMonth() + 1;
            return m < 10 ? '0' + m : m.toString();
        })(),
        m: (date.getMonth() + 1).toString(),
        mmm: (() => {
            // Return i18n('MONTH_JAN'), i18n('MONTH_FEB'), etc.
            const MONTHS = [
                'MONTH_JAN',
                'MONTH_FEB',
                'MONTH_MAR',
                'MONTH_APR',
                'MONTH_MAY',
                'MONTH_JUN',
                'MONTH_JUL',
                'MONTH_AUG',
                'MONTH_SEP',
                'MONTH_OCT',
                'MONTH_NOV',
                'MONTH_DEC'
            ] as const;
            return i18n(MONTHS[date.getMonth()]);
        })(),
        mmmm: (() => {
            // Return i18n('MONTH_JAN_LONG'), i18n('MONTH_FEB_LONG'), etc.
            const MONTHS = [
                'MONTH_JAN_LONG',
                'MONTH_FEB_LONG',
                'MONTH_MAR_LONG',
                'MONTH_APR_LONG',
                'MONTH_MAY_LONG',
                'MONTH_JUN_LONG',
                'MONTH_JUL_LONG',
                'MONTH_AUG_LONG',
                'MONTH_SEP_LONG',
                'MONTH_OCT_LONG',
                'MONTH_NOV_LONG',
                'MONTH_DEC_LONG'
            ] as const;
            return i18n(MONTHS[date.getMonth()]);
        })(),
        dd: (() => {
            const d = date.getDate();
            return d < 10 ? '0' + d : d.toString();
        })(),
        d: date.getDate().toString(),
        '24HH': (() => {
            const h = date.getHours();
            return h < 10 ? '0' + h : h.toString();
        })(),
        '24H': date.getHours().toString(),
        '12HH': (() => {
            // Needs to go from 12 to 12 and 01 to 11
            const hours = date.getHours();
            const actualHour = hours > 12 ? hours - 12 : hours === 0 ? 12 : hours;
            return actualHour < 10 ? '0' + actualHour : actualHour.toString();
        })(),
        '12H': (() => {
            // Needs to go from 12 to 12 and 1 to 11
            const hours = date.getHours();
            const actualHour = hours > 12 ? hours - 12 : hours === 0 ? 12 : hours;
            return actualHour.toString();
        })(),
        HH: (() => {
            // Needs to go from 00 to 12 and 01 to 11
            const hours = date.getHours();
            const actualHour = hours > 12 ? hours - 12 : hours;
            return actualHour < 10 ? '0' + actualHour : actualHour.toString();
        })(),
        H: (() => {
            // Needs to go from 0 to 12 and 1 to 11
            const hours = date.getHours();
            const actualHour = hours > 12 ? hours - 12 : hours;
            return actualHour.toString();
        })(),
        MM: (() => {
            const m = date.getMinutes();
            return m < 10 ? '0' + m : m.toString();
        })(),
        M: date.getMinutes().toString(),
        SS: (() => {
            const s = date.getSeconds();
            return s < 10 ? '0' + s : s.toString();
        })(),
        S: date.getSeconds().toString(),
        AMPM: date.getHours() < 12 ? 'AM' : 'PM',
        ampm: date.getHours() < 12 ? 'am' : 'pm'
    };
}