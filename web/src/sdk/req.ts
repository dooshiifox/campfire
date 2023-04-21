import { UnhandledReqError, type ApiResponse, type Handler } from "$/sdk/types";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type AnyRequest = SdkRequest<any, any>;

export class SdkRequest<
    ServerResponse,
    Response,
> {
    protected fetch: () => Promise<ApiResponse<ServerResponse>>;
    protected mapResponse: (data: ServerResponse) => Response;
    protected handlers: Record<string, [(errMsg: string) => boolean, Handler<[data: unknown]>]> = {};
    protected response: ApiResponse<ServerResponse> | null = null;

    constructor(fetch: () => Promise<ApiResponse<ServerResponse>>, map: (data: ServerResponse) => Response = (data) => data as unknown as Response) {
        this.fetch = fetch;
        this.mapResponse = map;
    }

    register<M extends string>(
        message: M,
        handler: Handler<[data: unknown]>
    ): SdkRequest<ServerResponse, Response>;
    register<M extends string>(
        name: M,
        code: string,
        handler: Handler<[data: unknown]>
    ): SdkRequest<ServerResponse, Response>;
    register<M extends string>(
        name_or_message: M,
        code_or_handler: string | Handler<[data: unknown]>,
        handler?: Handler<[data: unknown]>
    ): SdkRequest<ServerResponse, Response> {
        if (handler) {
            this.handlers[name_or_message] = [(msg) => msg === code_or_handler, handler];
            return this;
        }
        this.handlers[name_or_message] = [(msg) => msg === name_or_message, code_or_handler as Handler<[data: unknown]>];
        return this;
    }

    private async request(): Promise<ApiResponse<ServerResponse>> {
        if (this.response !== null) {
            return this.response;
        }

        try {
            this.response = await this.fetch();
        } catch (e) {
            console.warn("SDK: Unhandled error; Please report this bug to us!", e);
            this.response = {
                error: true,
                message: "FETCH:UnknownError",
                data: e instanceof Error ? e.message : e,
            }
        }

        if (this.response === null) {
            throw new UnhandledReqError("`this.response` is null even though it should be set by now...");
        }

        return this.response;
    }

    /** Sends the request and checks against any error handlers. */
    async send(): Promise<Response | undefined>;
    /** Sends the request but doesn't check against any error handlers.
     *  Will still map the result to the correct type.
     */
    async send(full: true): Promise<ApiResponse<Response>>;
    /** Sends the request and checks against any error handlers.
     *  Will not map the result to the correct type (i.e., stays as a
     *  `ServerResponse` instead of mapping to a `Response`).
     */
    async send(full: false, raw: true): Promise<ServerResponse | undefined>;
    /** Sends the request but doesn't check against any error handlers.
     *  Will not map the result to the correct type (i.e., stays as a
     *  `ServerResponse` instead of mapping to a `Response`).
     */
    async send(full: true, raw: true): Promise<ApiResponse<ServerResponse>>;
    async send(full = false, raw = false) {
        const response = await this.request();

        if (full) {
            if (raw) return response;
            if (response.error) return response;
            return {
                ...response,
                data: this.mapResponse(response.data),
            }
        }

        // The response was a success, return it.
        if (!response.error) {
            if (raw) return response.data;
            return this.mapResponse(response.data);
        }

        // For each registered error handler, find one that matches the
        // error message and run it.
        for (const [name, [matcher, handler]] of Object.entries(
            this.handlers
        )) {
            // Handle the 'any' case later
            if (name === "any") continue;

            if (matcher(response.message)) {
                handler(response.message, response.data);
                return;
            }
        }

        // Error currently unhandled.
        // If there is a handler that matches any error, run it.
        const anyError = this.handlers["any"];
        if (anyError) {
            anyError[1](response.message, response.data);
            return;
        }

        // Was a failure, didn't match an error handler, and no `any` handler
        // registered, meaning the user intentionally broke the typescript
        // rules here. They deserve this error tbh.
        console.warn("Unhandled SDK error:", response.message);
        throw new UnhandledReqError(response.message);
    }
}

export interface AnyErrors<R extends AnyRequest> {
    onError(handler: Handler<[data: unknown]>): R;
    emptyOnError(): R;
    throwOnError(): R;
}
export const anyErrors = <R extends AnyRequest>(req: R) => ({
    onError(handler: Handler<[data: unknown]>) {
        req.register("any", handler);
        return req;
    },
    emptyOnError() {
        req.register("any", () => undefined);
        return req;
    },
    throwOnError() {
        req.register("any", (msg, data) => {
            throw new Error(`SDK Error: ${msg} - ${JSON.stringify(data)}`);
        });
        return req;
    },
});

export interface JsonErrors<T> {
    onJsonPayloadTooLarge(handler: Handler<[{ length?: number, limit: number }]>): T;
    onJsonInvalidContentType(handler: Handler): T;
    onJsonUnknownDeserializeError(handler: Handler<[error: string]>): T;
    onJsonUnknownSerializeError(handler: Handler<[error: string]>): T;
    onJsonUnknownErrorReadingPayload(handler: Handler<[error: string]>): T;
    onJsonUnknownError(handler: Handler): T;
    onJsonError(handler: Handler): T;
}
export const jsonErrors = (req: AnyRequest) => ({
    onJsonPayloadTooLarge(handler: Handler<[{ length?: number, limit: number }]>) {
        req.register("JSON:PayloadTooLarge", (msg, data) => handler(msg, data as { length?: number, limit: number }));
        return this;
    },
    onJsonInvalidContentType(handler: Handler) {
        req.register("JSON:InvalidContentType", handler);
        return this;
    },
    onJsonUnknownDeserializeError(handler: Handler<[error: string]>) {
        req.register("JSON:UnknownDeserializeError", (msg, data) => handler(msg, data as string));
        return this;
    },
    onJsonUnknownSerializeError(handler: Handler<[error: string]>) {
        req.register("JSON:UnknownSerializeError", (msg, data) => handler(msg, data as string));
        return this;
    },
    onJsonUnknownErrorReadingPayload(handler: Handler<[error: string]>) {
        req.register("JSON:UnknownErrorReadingPayload", (msg, data) => handler(msg, data as string));
        return this;
    },
    onJsonUnknownError(handler: Handler) {
        req.register("JSON:UnknownError", handler);
        return this;
    },
    onJsonError(handler: Handler) {
        this.onJsonPayloadTooLarge(handler);
        this.onJsonInvalidContentType(handler);
        this.onJsonUnknownDeserializeError(handler);
        this.onJsonUnknownSerializeError(handler);
        this.onJsonUnknownErrorReadingPayload(handler);
        this.onJsonUnknownError(handler);
        return this;
    }
})

export interface AuthErrors<T> {
    onNoAuthToken(handler: Handler): T;
    onBadAuthToken(handler: Handler): T;
    onInvalidAuthToken(handler: Handler): T;
    onAuthError(handler: Handler): T;
}
export const authErrors = (req: AnyRequest) => ({
    onNoAuthToken(handler: Handler) {
        req.register("NoAuthToken", handler);
        return this;
    },
    onBadAuthToken(handler: Handler) {
        req.register("BadAuthToken", handler);
        return this;
    },
    onInvalidAuthToken(handler: Handler) {
        req.register("InvalidAuthToken", handler);
        return this;
    },
    onAuthError(handler: Handler) {
        this.onNoAuthToken(handler);
        this.onBadAuthToken(handler);
        this.onInvalidAuthToken(handler);
        return this;
    }
})

export interface ServerErrors<T> {
    onInternalServerError(handler: Handler): T;
    onEndpointNotFound(handler: Handler): T;
    onMethodNotAllowed(handler: Handler<[permittedMethods: string[]]>): T;
    onServerError(handler: Handler): T;
}
export const serverErrors = (req: AnyRequest) => ({
    onInternalServerError(handler: Handler) {
        req.register("InternalServerError", handler);
        return this;
    },
    onEndpointNotFound(handler: Handler) {
        req.register("EndpointNotFound", handler);
        return this;
    },
    onMethodNotAllowed(handler: Handler<[permittedMethods: string[]]>) {
        req.register("MethodNotAllowed", (msg, data) => {
            // `data` is returned as `Permitted: GET, POST, PUT, ...`
            // so we need to split it and remove the `Permitted: ` part.
            const permittedMethods = (data as string).replace("Permitted: ", "").split(", ");
            handler(msg, permittedMethods);
        });
        return this;
    },
    onServerError(handler: Handler) {
        this.onInternalServerError(handler);
        this.onEndpointNotFound(handler);
        this.onMethodNotAllowed(handler);
        return this;
    }
});

export interface FetchErrors<T> {
    onNetworkError(handler: Handler): T;
    onReturnsNotJson(handler: Handler<[response: string]>): T;
    onReturnsUnexpectedType(handler: Handler<[json: unknown]>): T;
    onReturnsUnknownError(handler: Handler<[error: unknown]>): T;
    onFetchError(handler: Handler): T;
}
export const fetchErrors = (req: AnyRequest) => ({
    onNetworkError(handler: Handler) {
        req.register("FETCH:NetworkError", handler);
        return this;
    },
    onReturnsNotJson(handler: Handler<[response: string]>) {
        req.register("FETCH:NotJson", (msg, data) => handler(msg, data as string));
        return this;
    },
    onReturnsUnexpectedType(handler: Handler<[json: unknown]>) {
        req.register("FETCH:UnexpectedType", handler);
        return this;
    },
    onReturnsUnknownError(handler: Handler<[error: unknown]>) {
        req.register("FETCH:UnknownError", handler);
        return this;
    },
    onFetchError(handler: Handler) {
        this.onNetworkError(handler);
        this.onReturnsNotJson(handler);
        this.onReturnsUnexpectedType(handler);
        this.onReturnsUnknownError(handler);
        return this;
    }
});

export const endpoint = "http://localhost:8080"

export type Method = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";
export async function fetchRequest<ServerResponse>(uri: string, method: Method, req?: { body?: unknown; authToken?: string }): Promise<ApiResponse<ServerResponse>> {
    let resp: Response;
    try {
        const headers: Record<string, string> = {
            "Content-Type": "application/json",
            "Accept": "application/json",
            "Access-Control-Allow-Origin": "*"
        };

        if (req?.authToken) {
            headers["Authorization"] = `Bearer ${req.authToken}`;
        }

        resp = await fetch(endpoint + uri, {
            method: method,
            headers,
            body: req?.body ? JSON.stringify(req.body) : undefined,
        });
    } catch (e) {
        return {
            error: true,
            message: "FETCH:NetworkError",
            data: null,
        }
    }

    // Try parse the JSON, but save it to a string first
    const text = await resp.text();
    let json: unknown;
    try {
        json = JSON.parse(text);
    } catch (e) {
        console.warn(
            `Could not parse server response from \`${method} ${uri}\`:\nBody\n    ${JSON.stringify(
                req?.body,
                null,
                4
            ).replace(
                /\n/g,
                "\n    "
            )}\n\nResponse\n    ${text}`
        );

        return {
            error: true,
            message: "FETCH:NotJson",
            data: text,
        }
    }

    if (typeof json !== "object" || json === null || !("error" in json)) {
        return {
            error: true,
            message: "FETCH:UnexpectedType",
            data: json,
        }
    }

    // Just assume it's the correct type.
    return json as ApiResponse<ServerResponse>;
}
