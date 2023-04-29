import { UnhandledReqError, type ApiResponse, type Handler, ThrownReqError } from '$/sdk/types';
import { PUBLIC_ENDPOINT } from '$env/static/public';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
/** Utility type for grouped error registers (e.g., `anyErrors`, `serverErrors`)
 *  that don't care about the request types.
 */
export type AnyRequest = SdkRequest<any, any>;

export class SdkRequest<ServerResponse, Response> {
	/** The callback that fetches the content from the server. */
	protected fetch: () => Promise<ApiResponse<ServerResponse>>;
	/** Map the server response to the SDK response.
	 *
	 *  This is called before any error handlers are called.
	 *  It will not run if `send` is called with `raw` set to `true`.
	 */
	protected mapResponse: (resp: ApiResponse<ServerResponse>) => ApiResponse<Response>;
	/** All error handlers.
	 *
	 *  A mapping of the handler id to a tuple of the error matcher
	 *  (the callback that determines if the handler should be called) and the
	 *  handler itself.
	 */
	protected handlers: Record<
		string,
		[(errCode: string, data: unknown) => boolean, Handler<[data: unknown]>]
	> = {};
	/** Ignore a handler when trying to register an error that's already been registered.
	 *
	 *  With this `false`, you could do this:
	 *  ```
	 *  req.register("MyError" () => console.log("hi"));
	 *  req.register("MyError" () => console.log("oh no!"));
	 *  ```
	 *  and only the last registered handler would be called ("oh no!")
	 *
	 *  ```
	 *  req.register("MyError" () => console.log("hi"));
	 *  req.ignoreHandlerDuplicates = true;
	 *  req.register("MyError" () => console.log("oh no!"));
	 *  req.ignoreHandlerDuplicates = false;
	 *  ```
	 *  and the first handler would be called ("hi") because it won't be overwritten.
	 */
	protected ignoreHandlerDuplicates: boolean = false;
	/** The cached server response. */
	protected responsePromise: Promise<ApiResponse<ServerResponse>> | null = null;

	/** Construct a new SdkRequest.
	 *
	 *  Requires a callback that will fetch the content from the server. Also
	 *  supports additional hooks that can map the response.
	 */
	constructor(
		fetch: () => Promise<ApiResponse<ServerResponse>>,
		extra?: {
			/** Maps a successful response.
			 *
			 *  Will not be run when `raw` is set while calling `send`.
			 */
			map?: (data: ServerResponse) => Response;
			/** Maps the full server response. Useful for mapping an
			 *  error into a success, or vice versa.
			 *
			 *  Will not be run when `raw` is set while calling `send`.
			 */
			mapFull?: (resp: ApiResponse<ServerResponse>) => ApiResponse<Response>;
		}
	) {
		this.fetch = fetch;
		this.mapResponse =
			extra?.mapFull ??
			((resp) => {
				if (resp.error) {
					return resp;
				} else {
					return {
						...resp,
						data: extra?.map ? extra.map(resp.data) : (resp.data as unknown as Response)
					};
				}
			});
	}

	/** Registers an error handler with a `code` that acts as the ID and also
	 *  to determine if the handler should be called, and the `handler` itself.
	 *
	 *  The `code` must match the `code` the server sends exactly.
	 *
	 *  Only one error handler will be called per request. If you `send`
	 *  a request with `full` set, no handlers will be called.
	 */
	register<M extends string>(
		code: M,
		handler: Handler<[data: unknown]>
	): SdkRequest<ServerResponse, Response>;
	/** Registers an error handler with an `id`, a `code` to determine if the
	 *  handler should be called, and the `handler` itself.
	 *
	 *  The `code` must match the `code` the server sends exactly.
	 *
	 *  Only one error handler will be called per request. If you `send`
	 *  a request with `full` set, no handlers will be called.
	 */
	register<M extends string>(
		id: M,
		code: string,
		handler: Handler<[data: unknown]>
	): SdkRequest<ServerResponse, Response>;
	/** Registers an error handler with an `id`, a `matcher` to determine if the
	 *  handler should be called, and the `handler` itself.
	 *
	 *  The `matcher` is called with both the error code and the data returned
	 *  with it. It must return a boolean indicating if the handler should be
	 *  called.
	 *
	 *  Only one error handler will be called per request. If you `send`
	 *  a request with `full` set, no handlers will be called.
	 */
	register<M extends string>(
		id: M,
		matcher: (code: string, data: unknown) => boolean,
		handler: Handler<[data: unknown]>
	): SdkRequest<ServerResponse, Response>;
	register<M extends string>(
		id_or_code: M,
		code_or_handler_or_matcher:
			| string
			| ((code: string, data: unknown) => boolean)
			| Handler<[data: unknown]>,
		handler?: Handler<[data: unknown]>
	): SdkRequest<ServerResponse, Response> {
		if (handler) {
			this.handlers[id_or_code] = [
				typeof code_or_handler_or_matcher === 'string'
					? (code) => code === code_or_handler_or_matcher
					: (code_or_handler_or_matcher as (code: string, data: unknown) => boolean),
				handler
			];
			return this;
		}
		this.handlers[id_or_code] = [
			(code) => code === id_or_code,
			code_or_handler_or_matcher as Handler<[data: unknown]>
		];
		return this;
	}

	/** Wraps the callback such that no existing error handlers will be
	 *  overwritten. Useful for aliases that register multiple handlers,
	 *  e.g. `onFetchError` or `onAuthError`.
	 *
	 *  Will return `ignoreHandlerDuplicates` to it's original state after the
	 *  callback is executed, so if it is already set this function will
	 *  keep it at `true`.
	 */
	ignoreDuplicates(cb: () => void) {
		const previousState = this.ignoreHandlerDuplicates;
		this.ignoreHandlerDuplicates = true;
		cb();
		this.ignoreHandlerDuplicates = previousState;
	}

	/** Calls the `fetch`er, or returns from cache if it's already been sent,
	 *  returning an `ApiResponse<ServerResponse>`.
	 */
	private async request(): Promise<ApiResponse<ServerResponse>> {
		try {
			// Return from cache if we've already sent the request
			if (this.responsePromise !== null) {
				return await this.responsePromise;
			}

			this.responsePromise = this.fetch();
			await this.responsePromise;
		} catch (e) {
			console.warn('Campfire SDK: Unhandled error; Please report this bug to us!', e);
			this.responsePromise = Promise.resolve({
				error: true,
				code: 'FETCH:UnknownError',
				data: e instanceof Error ? e.message : e
			});
		}

		if (this.responsePromise === null) {
			throw new UnhandledReqError(
				'StupidSdkError',
				"Something went horrifically wrong in the Campfire SDK considering we *just* set this variable and it's still null"
			);
		}

		return this.responsePromise;
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
	/** Sends the request.
	 *  - `full` - Whether to run error handlers or return the full ApiResponse.
	 *  - `raw`  - Whether to map the response to the correct SDK type or leave
	 *             it how the server returned it.
	 */
	async send(full = false, raw = false) {
		const serverResponse = await this.request();
		const response = raw ? serverResponse : this.mapResponse(serverResponse);

		if (full) {
			return response;
		}

		// The response was a success, return it.
		if (!response.error) {
			return response.data;
		}

		// For each registered error handler, find one that matches the
		// error code and run it.
		for (const [id, [matcher, handler]] of Object.entries(this.handlers)) {
			// Handle the 'any' case later
			if (id === 'any') continue;

			if (matcher(response.code, response.data)) {
				handler(response.code, response.data);
				return;
			}
		}

		// Error currently unhandled.
		// If there is a handler that matches any error, run it.
		const anyError = this.handlers['any'];
		if (anyError) {
			anyError[1](response.code, response.data);
			return;
		}

		// Was a failure, didn't match an error handler, and no `any` handler
		// registered, meaning the user intentionally broke the typescript
		// rules here. They deserve this error tbh.
		console.warn('Unhandled Campfire SDK error:', response.code);
		throw new UnhandledReqError(response.code, response.data);
	}

	/** A helper function for returning both `this.send()` and
	 *  `this.send(true, true)`. Only useful for tests.
	 */
	async sendRawFullAndHandled(): Promise<[ApiResponse<ServerResponse>, Response | undefined]> {
		const response = await this.send(true, true);
		return [response, await this.send()];
	}
}

/////////////////////////////
//     REQUEST FETCHING
/////////////////////////////

// Get the `PUBLIC_ENDPOINT` value from the env
/** The endpoint all requests should be made against. */
export const endpoint = PUBLIC_ENDPOINT ?? 'http://localhost:8080';

/** A valid HTTP method. */
export type Method =
	| 'GET'
	| 'POST'
	| 'PUT'
	| 'DELETE'
	| 'PATCH'
	| 'HEAD'
	| 'CONNECT'
	| 'OPTIONS'
	| 'TRACE';

/** Additional information about a request. */
export type RequestInfo = {
	/** The body of the request. */
	body?: unknown;
	/** The auth token to use for the request, if authentication is required.
	 *
	 *  Gets sent as a `Bearer` token in the `Authorization` header.
	 */
	authToken?: string;
};

/** Fetches a new request from a server.
 *
 *  @param uri    - The URI to send the request to, e.g., `/v1/users`.
 *  @param method - The HTTP method to use, e.g., `GET` or `POST`.
 *  @param req    - Additional information about the request.
 */
export async function fetchRequest<ServerResponse>(
	uri: string,
	method: Method,
	req?: { body?: unknown; authToken?: string }
): Promise<ApiResponse<ServerResponse>> {
	let resp: Response;
	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};

		if (req?.authToken) {
			headers['Authorization'] = `Bearer ${req.authToken}`;
		}

		resp = await fetch(endpoint + uri, {
			method: method,
			headers,
			body: req?.body ? JSON.stringify(req.body) : undefined
		});
	} catch (e) {
		return {
			error: true,
			code: 'FETCH:NetworkError',
			data: null
		};
	}

	// Try parse the JSON, but save it to a string first so we can log it if
	// it fails (you can't read the body of a response twice)
	const text = await resp.text();
	let json: unknown;
	try {
		json = JSON.parse(text);
	} catch (e) {
		console.warn(
			`Campfire SDK: Could not parse server response from \`${method} ${uri}\`:\nBody\n    ${JSON.stringify(
				req?.body,
				null,
				4
			).replace(/\n/g, '\n    ')}\n\nResponse\n    ${text}`
		);

		return {
			error: true,
			code: 'FETCH:NotJson',
			data: text
		};
	}

	if (typeof json !== 'object' || json === null || !('error' in json)) {
		return {
			error: true,
			code: 'FETCH:UnexpectedType',
			data: json
		};
	}

	// Just assume it's the correct type.
	return json as ApiResponse<ServerResponse>;
}

/////////////////////////////
//      ERROR GROUPS
/////////////////////////////

export interface AnyErrors<R extends AnyRequest> {
	/** Register a handler for any uncaught error. */
	onError(handler: Handler<[data: unknown]>): R;
	/** Return `undefined` when an error goes uncaught. */
	emptyOnError(): R;
	/** Throw when an error goes uncaught. */
	throwOnError(): R;
}
export const anyErrors = <R extends AnyRequest>(req: R) => ({
	onError(handler: Handler<[data: unknown]>) {
		req.register('any', handler);
		return req;
	},
	emptyOnError() {
		req.register('any', () => {});
		return req;
	},
	throwOnError() {
		req.register('any', (code, data) => {
			throw new ThrownReqError(code, data);
		});
		return req;
	}
});

export interface JsonErrors<T> {
	/** The JSON payload was too large and the server refused to decode it, or
	 *  no `Content-Length` header was provided and the server refused to decode
	 *  it.
	 *
	 *  In the event no `Content-Length` header was provided, it only returns
	 *  the `limit` and not the `length` (because the server didn't know it).
	 *
	 *  If you're using the SDK and the `Content-Length` error occurs, there is
	 *  either a problem in the `fetch` implementation of your environment or
	 *  there is an SDK bug. Please file a report if you encounter this either way.
	 *
	 *  These should be seperated into two different errors. TODO (?)
	 */
	onJsonPayloadTooLarge(handler: Handler<[{ length?: number; limit: number }]>): T;
	/** The content type of the request was not accepted.
	 *
	 *  This value should always be `application/json`. If you're using the SDK,
	 *  this problem should not occur - Please file a report if it does.
	 */
	onJsonInvalidContentType(handler: Handler): T;
	/** A problem occured deserializing your JSON request.
	 *
	 *  This probably means your JSON was invalid. Run it through a JSON
	 *  validator to find out. If you're using the SDK, this problem should not
	 *  occur - Please file a report if it does.
	 */
	onJsonUnknownDeserializeError(handler: Handler<[error: string]>): T;
	/** A problem occured serializing your JSON response into a format the
	 *  server is happy with. This is probably a bug in the SDK, assuming you're
	 *  using it correctly. Please file a report if you encounter this and you
	 *  know the types you're sending are correct.
	 */
	onJsonUnknownSerializeError(handler: Handler<[error: string]>): T;
	/** A problem occured reading the request payload. Please file a report if
	 *  you encounter this and weren't intentionally trying to break stuff.
	 */
	onJsonUnknownErrorReadingPayload(handler: Handler<[error: string]>): T;
	/** The backend Rust enum for JSON payload deserializing has the
	 *  `non_exhaustive` attribute. All errors as of writing this are handled
	 *  in one of the above errors. If you encounter this error, please file a
	 *  report, as it means a new error variant was added and we need to handle
	 *  it.
	 */
	onJsonUnknownError(handler: Handler): T;
	/** A server JSON error occured.
	 *
	 *  Alias for registering all unregistered `JsonErrors` with the same callback.
	 */
	onJsonError(handler: Handler): T;
}
export const jsonErrors = (req: AnyRequest) => ({
	onJsonPayloadTooLarge(handler: Handler<[{ length?: number; limit: number }]>) {
		req.register('JSON:PayloadTooLarge', (code, data) =>
			handler(code, data as { length?: number; limit: number })
		);
		return this;
	},
	onJsonInvalidContentType(handler: Handler) {
		req.register('JSON:InvalidContentType', handler);
		return this;
	},
	onJsonUnknownDeserializeError(handler: Handler<[error: string]>) {
		req.register('JSON:UnknownDeserializeError', (code, data) => handler(code, data as string));
		return this;
	},
	onJsonUnknownSerializeError(handler: Handler<[error: string]>) {
		req.register('JSON:UnknownSerializeError', (code, data) => handler(code, data as string));
		return this;
	},
	onJsonUnknownErrorReadingPayload(handler: Handler<[error: string]>) {
		req.register('JSON:UnknownErrorReadingPayload', (code, data) => handler(code, data as string));
		return this;
	},
	onJsonUnknownError(handler: Handler) {
		req.register('JSON:UnknownError', handler);
		return this;
	},
	onJsonError(handler: Handler) {
		req.ignoreDuplicates(() => {
			this.onJsonPayloadTooLarge(handler);
			this.onJsonInvalidContentType(handler);
			this.onJsonUnknownDeserializeError(handler);
			this.onJsonUnknownSerializeError(handler);
			this.onJsonUnknownErrorReadingPayload(handler);
			this.onJsonUnknownError(handler);
		});
		return this;
	}
});

export interface AuthErrors<T> {
	/** No auth token was supplied in the request, but one was required. */
	onNoAuthToken(handler: Handler): T;
	/** The auth token could not be decoded.
	 *
	 *  Each auth token is a JWT with a number that pairs up to the database.
	 *  This error means that the token could not be decoded, likely because it
	 *  was tampered with or corrupted.
	 */
	onBadAuthToken(handler: Handler): T;
	/** The supplied auth token was rejected, either because it expired or
	 *  because it simply never existed.
	 */
	onInvalidAuthToken(handler: Handler): T;
	/** An authorization error occured.
	 *
	 *  Alias for registering all unregistered `AuthErrors` with the same callback.
	 */
	onAuthError(handler: Handler): T;
}
export const authErrors = (req: AnyRequest) => ({
	onNoAuthToken(handler: Handler) {
		req.register('NoAuthToken', handler);
		return this;
	},
	onBadAuthToken(handler: Handler) {
		req.register('BadAuthToken', handler);
		return this;
	},
	onInvalidAuthToken(handler: Handler) {
		req.register('InvalidAuthToken', handler);
		return this;
	},
	onAuthError(handler: Handler) {
		req.ignoreDuplicates(() => {
			this.onNoAuthToken(handler);
			this.onBadAuthToken(handler);
			this.onInvalidAuthToken(handler);
		});
		return this;
	}
});

export interface ServerErrors<T> {
	/** An internal server error occured. Most of the time this just means
	 *  something couldn't be inserted into the database. If you encounter this
	 *  error, please report it to us.
	 */
	onInternalServerError(handler: Handler): T;
	/** The requested endpoint does not exist. This is either an SDK or server
	 *  error, so please report it to us.
	 */
	onEndpointNotFound(handler: Handler): T;
	/** The requested endpoint exists, but the method you're using is not
	 *  allowed. This is either an SDK or server error, so please report it to
	 *  us.
	 */
	onMethodNotAllowed(handler: Handler<[permittedMethods: string[]]>): T;
	/** A server error occured.
	 *
	 *  Alias for registering all unregistered `ServerErrors` with the same callback.
	 */
	onServerError(handler: Handler): T;
}
export const serverErrors = (req: AnyRequest) => ({
	onInternalServerError(handler: Handler) {
		req.register('InternalServerError', handler);
		return this;
	},
	onEndpointNotFound(handler: Handler) {
		req.register('EndpointNotFound', handler);
		return this;
	},
	onMethodNotAllowed(handler: Handler<[permittedMethods: string[]]>) {
		req.register('MethodNotAllowed', (code, data) => {
			// `data` is returned as `Permitted: GET, POST, PUT, ...`
			// so we need to split it and remove the `Permitted: ` part.
			const permittedMethods = (data as string).replace('Permitted: ', '').split(', ');
			handler(code, permittedMethods);
		});
		return this;
	},
	onServerError(handler: Handler) {
		req.ignoreDuplicates(() => {
			this.onInternalServerError(handler);
			this.onEndpointNotFound(handler);
			this.onMethodNotAllowed(handler);
		});
		return this;
	}
});

export interface FetchErrors<T> {
	/** The user does not have internet connection or a CORS error occured. */
	// TODO: Find a way to seperate CORS errors from network errors.
	onNetworkError(handler: Handler): T;
	/** The response from the server was not JSON. Please report this to us, as
	 *  under no circumstance should this error occur.
	 */
	onReturnsNotJson(handler: Handler<[response: string]>): T;
	/** The response from the server was JSON, but it was not an `ApiResponse`
	 *  and as such couldn't be handled by the SDK. Please report this to us, as
	 *  under no circumstance should this error occur.
	 */
	onReturnsUnexpectedType(handler: Handler<[json: unknown]>): T;
	/** The fetch function threw an error. This is an SDK error, so please
	 *  report it to us.
	 */
	onReturnsUnknownError(handler: Handler<[error: unknown]>): T;
	/** A fetch error occured.
	 *
	 *  Alias for registering all unregistered `FetchErrors` with the same callback.
	 */
	onFetchError(handler: Handler): T;
}
export const fetchErrors = (req: AnyRequest) => ({
	onNetworkError(handler: Handler) {
		req.register('FETCH:NetworkError', handler);
		return this;
	},
	onReturnsNotJson(handler: Handler<[response: string]>) {
		req.register('FETCH:NotJson', (code, data) => handler(code, data as string));
		return this;
	},
	onReturnsUnexpectedType(handler: Handler<[json: unknown]>) {
		req.register('FETCH:UnexpectedType', handler);
		return this;
	},
	onReturnsUnknownError(handler: Handler<[error: unknown]>) {
		req.register('FETCH:UnknownError', handler);
		return this;
	},
	onFetchError(handler: Handler) {
		req.ignoreDuplicates(() => {
			this.onNetworkError(handler);
			this.onReturnsNotJson(handler);
			this.onReturnsUnexpectedType(handler);
			this.onReturnsUnknownError(handler);
		});
		return this;
	}
});
