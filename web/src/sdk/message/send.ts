import {
    SdkRequest, fetchRequest,
    type FetchErrors, fetchErrors,
    type ServerErrors, serverErrors,
    type JsonErrors, jsonErrors,
    type AuthErrors, authErrors,
    type AnyErrors, anyErrors,
} from "$/sdk/req";
import type { Handler, Snowflake } from "$/sdk/types";

export type SendBody = {
    content: string;
    channelId: Snowflake;
};

export type SendResponse = {
    message_id: Snowflake;
};

export type Send = {
    onMessageTooShort(handler: Handler<[minLength: number]>): Send;
    onMessageTooLong(handler: Handler<[maxLength: number]>): Send;
    onChannelNotFound(handler: Handler): Send;
} & FetchErrors<Send> & ServerErrors<Send> & JsonErrors<Send> & AuthErrors<Send> & AnyErrors<SdkRequest<SendResponse, SendResponse>>;

export function send(authToken: string, body: SendBody) {
    const req = new SdkRequest<SendResponse, SendResponse>(() =>
        fetchRequest<SendResponse>("/message/" + body.channelId, "POST", {
            body: {
                content: body.content,
            },
            authToken,
        })
    );

    return {
        onMessageTooShort(handler: Handler<[minLength: number]>) {
            req.register("MessageTooShort", (msg) => handler(msg, 0))
            return this;
        },
        onMessageTooLong(handler: Handler<[maxLength: number]>) {
            req.register("MessageTooLong", (msg, data) => handler(msg, data as number))
            return this;
        },
        onChannelNotFound(handler: Handler) {
            req.register("ChannelNotFound", handler)
            return this;
        },
        ...anyErrors(req),
        ...fetchErrors(req),
        ...serverErrors(req),
        ...jsonErrors(req),
        ...authErrors(req),
    } as unknown as Send;
};
