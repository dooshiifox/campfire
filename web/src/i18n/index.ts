import { get, writable } from "svelte/store";
import { en } from "$/i18n/lang/en";
import { uwu } from "$/i18n/lang/uwu";

export type I18nKey =
    | "EMPTY"
    // {key} - The translation key that couldn't be found
    | "TRANSLATION_NOT_FOUND"
    | "ERROR"
    | "WARNING"
    | "INFO"
    | "NETWORK_ERROR"
    | "NOT_LOGGED_IN"
    | "SESSION_CORRUPTED"
    | "SESSION_EXPIRED"
    | "LOGIN"
    | "LOGOUT"
    // {user} - The user's username
    // {discrim} - The user's discriminator
    | "LOGGED_IN_AS"
    | I18nKey_Date
    | "SERVER_ERROR_FETCHING_GUILDS"
    | "FETCH_ERROR_FETCHING_GUILDS"
    | "MISC_ERROR_FETCHING_GUILDS"
export type I18n = Record<I18nKey, string>;

const LANGUAGES = {
    "en": en,
    "uwu": uwu,
} satisfies Record<string, I18n>;
export const language = writable<keyof typeof LANGUAGES>("en");

export function i18n(key: I18nKey, args: Record<string, string> = {}): string {
    const lang = get(language);
    let str = LANGUAGES[lang][key];
    if (!str) {
        str = LANGUAGES[lang].TRANSLATION_NOT_FOUND;
        args.key = key;
    }
    for (const [key, value] of Object.entries(args)) {
        str = str.replace(new RegExp(`\\{${key}\\}`, "g"), value);
    }
    return str;
}

type I18nKey_Date =
    // Has access to...
    // {yyyy} - 4 digit year
    // {yy} - 2 digit year
    // {mm} - 2 digit month (01-12)
    // {m} - 1 digit month (1-12)
    // {mmm} - 3 letter month (Takes from `JAN`, `FEB`, etc.)
    // {mmmm} - full name month (Takes from `JAN_LONG`, `FEB_LONG`, etc.)
    // {dd} - 2 digit day (01-31)
    // {d} - 1 digit day (1-31)
    // {HH} - 2 digit hour (00-12 and 01-11)
    // {H} - 1 digit hour (0-12 and 1-11)
    // {12HH} - 2 digit hour (same as above but midnight is 12 instead of 00)
    // {12H} - 1 digit hour (same as above but midnight is 12 instead of 0)
    // {24HH} - 2 digit 24-hour (00-23)
    // {24H} - 1 digit 24-hour (0-23)
    // {MM} - 2 digit minute (00-59)
    // {M} - 1 digit minute (0-59)
    // {SS} - 2 digit second (00-59)
    // {S} - 1 digit second (0-59)
    // {AMPM} - "AM" or "PM"
    // {ampm} - "am" or "pm"
    | "DATE_FORMAT_TODAY"
    | "DATE_FORMAT_YESTERDAY"
    | "DATE_FORMAT"
    | "DATE_FORMAT_LONG"
    | "MONTH_JAN"
    | "MONTH_JAN_LONG"
    | "MONTH_FEB"
    | "MONTH_FEB_LONG"
    | "MONTH_MAR"
    | "MONTH_MAR_LONG"
    | "MONTH_APR"
    | "MONTH_APR_LONG"
    | "MONTH_MAY"
    | "MONTH_MAY_LONG"
    | "MONTH_JUN"
    | "MONTH_JUN_LONG"
    | "MONTH_JUL"
    | "MONTH_JUL_LONG"
    | "MONTH_AUG"
    | "MONTH_AUG_LONG"
    | "MONTH_SEP"
    | "MONTH_SEP_LONG"
    | "MONTH_OCT"
    | "MONTH_OCT_LONG"
    | "MONTH_NOV"
    | "MONTH_NOV_LONG"
    | "MONTH_DEC"
    | "MONTH_DEC_LONG"
