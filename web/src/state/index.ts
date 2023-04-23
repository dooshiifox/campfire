import { writable, type Writable } from "svelte/store";
import { Guild, type Type } from "$/sdk";
import { i18n, type I18nKey } from "$/i18n";
import { browser } from "$app/environment";
import type { User } from "$/sdk/types";

export type LoggedInUser = {
    accessToken: string;
    user: User;
};

const userStorage = browser ? localStorage.getItem("user") : null;
export const user = writable<LoggedInUser | undefined>(userStorage ? JSON.parse(userStorage) : undefined);

interface PopupError {
    severity: "error" | "warning" | "info";
    message: string;
    start: number;
    autoDismiss: boolean;
}
export const errors = writable<PopupError[]>([]);
function addError(message: I18nKey, autoDismiss: boolean = true) {
    errors.update((errors) => {
        const msg = i18n(message);

        // If there exists an error with the same message, remove that one
        errors = errors.filter((error) => error.message !== msg && error.severity !== "error" && (!error.autoDismiss || !autoDismiss));

        // Add the error
        errors.push({
            severity: "error",
            message: msg,
            start: Date.now(),
            autoDismiss,
        });
        return errors;
    });
};

export const guilds = writable<Type.Guild[]>([]);
async function fetchGuilds(accessToken?: string) {
    if (accessToken === "" || accessToken === undefined) {
        guilds.set([]);
        return;
    }

    const fetchedGuilds = await Guild.getJoined(accessToken).onBadAuthToken(() => {
        addError("SESSION_CORRUPTED");
    }).onInvalidAuthToken(() => {
        addError("SESSION_EXPIRED");
    }).onNoAuthToken(() => {
        addError("NOT_LOGGED_IN");
    }).onNetworkError(() => {
        addError("NETWORK_ERROR", false);
    }).onServerError(() => {
        addError("SERVER_ERROR_FETCHING_GUILDS");
    }).onFetchError(() => {
        addError("FETCH_ERROR_FETCHING_GUILDS");
    }).onJsonError(() => {
        addError("SERVER_ERROR_FETCHING_GUILDS");
    }).onError((err) => {
        console.warn("Unknown error while fetching guilds:", err);
        addError("MISC_ERROR_FETCHING_GUILDS");
    }).send();

    if (fetchedGuilds) {
        guilds.set(fetchedGuilds);
    }
}

user.subscribe((u) => {
    console.log("User changed:", u);
    fetchGuilds(u?.accessToken);
    if (browser) {
        if (u) {
            localStorage.setItem("user", JSON.stringify(u));
        } else {
            localStorage.removeItem("user");
        }
    }
});

// An implementation that stores drafts for different channels.
// To create/get a draft for a channel, use `messageDraft(channelId)`.
const drafts = new Map<Type.Snowflake, Writable<string>>();
export function messageDraft(channelId: string): Writable<string> {
    let draft = drafts.get(channelId);

    if (!draft) {
        draft = writable("");
        drafts.set(channelId, draft);
    }

    return draft;
}
