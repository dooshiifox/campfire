import { writable, type Writable } from "svelte/store";
import { Guild, type Type } from "$/sdk";
import { i18n, type I18nKey } from "$/i18n";

export const accessToken = writable("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0a24iOjU5NjIyODc0NDYzOTE3MDI0MDF9.3aUNBz2k2E0xQt7UIUzLw4iPa31XbrxCsJoXRmprV3k");

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
async function fetchGuilds(accessToken: string) {
    if (accessToken === "") {
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

accessToken.subscribe((accessToken) => {
    fetchGuilds(accessToken);
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
