/////////////////////////////
//   SERVER RESPONSE TYPES
/////////////////////////////

/** A JSON response from the server. */
export type ApiResponse<T> = ApiResponseSuccess<T> | ApiResponseError<string>;
/** A successful API response from the server. */
export type ApiResponseSuccess<T> = {
	error: false;
	data: T;
};
/** An unsuccessful API response from the server. */
export type ApiResponseError<M extends string, E = unknown> = {
	error: true;
	code: M;
	data: E;
};

/////////////////////////////
//       SDK TYPES
/////////////////////////////

/** An error handler for the SDK.
 *
 *  The first argument is the error message, and the rest are the arguments
 *  which can be specified by the function that registers it.
 */
export type Handler<Args extends unknown[] = []> = (msg: string, ...args: Args) => void;

/** A request that was not handled successfully because no
 *  `any` error handler was registered.
 */
export class UnhandledReqError extends Error {
	code: string;
	data: unknown;

	constructor(code: string, data: unknown) {
		super('Campfire SDK request error went unhandled and failed with code ' + code);
		this.code = code;
		this.data = data;
	}
}
/** A request that was not handled successfully because `throwOnError` was
 *  registered and no error handlers were matched.
 */
export class ThrownReqError extends Error {
	code: string;
	data: unknown;

	constructor(code: string, data: unknown) {
		super('Campfire SDK request failed with code ' + code);
		this.code = code;
		this.data = data;
	}
}

/////////////////////////////
//     SERVER TYPES
/////////////////////////////

export type Snowflake = string;

export interface Guild {
	id: Snowflake;
	owner: User;
	name: string;
	channels: Channel[];
}

export interface User {
	id: Snowflake;
	username: string;
	discrim: number;
	profile_img_id: Snowflake | null;
	accent_color: string | null;
	pronouns: string | null;
	bio: string | null;
}

export interface Channel {
	id: Snowflake;
	name: string;
}

export interface Message {
	id: Snowflake;
	channel_id: Snowflake;
	author: User;
	content: string;
	sent_at: number;
	updated_at: number;
}
