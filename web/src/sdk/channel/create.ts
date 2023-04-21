import {
    SdkRequest, fetchRequest,
    type FetchErrors, fetchErrors,
    type ServerErrors, serverErrors,
    type JsonErrors, jsonErrors,
    type AuthErrors, authErrors,
    type AnyErrors, anyErrors,
} from "$/sdk/req";
import type { Handler, Snowflake } from "$/sdk/types";

export type CreateBody = {
    name: string;
    place_before?: Snowflake;
};

export type CreateResponse = {
    id: Snowflake;
};

export type Create = {
    onNameTooShort(handler: Handler<[minLength: number]>): Create;
    onNameTooLong(handler: Handler<[maxLength: number]>): Create;
    onNameInvalid(handler: Handler<[regex: string]>): Create;
    onGuildNotFound(handler: Handler): Create;
    onPlaceBeforeNotFound(handler: Handler): Create;
    onPermissionDenied(handler: Handler): Create;
} & FetchErrors<Create> & ServerErrors<Create> & JsonErrors<Create> & AuthErrors<Create> & AnyErrors<SdkRequest<CreateResponse, CreateResponse>>;

export function create(authToken: string, body: CreateBody) {
    const req = new SdkRequest<CreateResponse, CreateResponse>(() =>
        fetchRequest<CreateResponse>("/channel/create", "POST", {
            body,
            authToken,
        })
    );

    return {
        onNameTooShort(handler: Handler<[minLength: number]>) {
            req.register("NameTooShort", (msg, data) => handler(msg, data as number));
            return this;
        },
        onNameTooLong(handler: Handler<[maxLength: number]>) {
            req.register("NameTooLong", (msg, data) => handler(msg, data as number));
            return this;
        },
        onNameInvalid(handler: Handler<[regex: string]>) {
            req.register("NameInvalid", (msg, data) => handler(msg, data as string));
            return this;
        },
        onGuildNotFound(handler: Handler) {
            req.register("GuildNotFound", handler);
            return this;
        },
        onPlaceBeforeNotFound(handler: Handler) {
            req.register("PlaceBeforeNotFound", handler);
            return this;
        },
        onPermissionDenied(handler: Handler) {
            req.register("PermissionDenied", handler);
            return this;
        },
        ...anyErrors(req),
        ...fetchErrors(req),
        ...serverErrors(req),
        ...jsonErrors(req),
        ...authErrors(req),
    } as unknown as Create;
};
