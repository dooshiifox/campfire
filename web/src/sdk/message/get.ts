import {
	SdkRequest,
	fetchRequest,
	type FetchErrors,
	fetchErrors,
	type ServerErrors,
	serverErrors,
	type JsonErrors,
	jsonErrors,
	type AuthErrors,
	authErrors,
	type AnyErrors,
	anyErrors
} from '$/sdk/req';
import type { Handler, Message, Snowflake } from '$/sdk/types';

export type GetBody = {
	channelId: Snowflake;
};

export type GetResponse = Message[];

export type Get = {
	onNotFound(handler: Handler): Get;
} & FetchErrors<Get> &
	ServerErrors<Get> &
	JsonErrors<Get> &
	AuthErrors<Get> &
	AnyErrors<SdkRequest<GetResponse, GetResponse>>;

export function get(authToken: string, params: GetBody): Get {
	const req = new SdkRequest<GetResponse, GetResponse>(() =>
		fetchRequest<GetResponse>('/message/' + params.channelId, 'GET', {
			authToken
		})
	);

	return {
		onNotFound(handler: Handler) {
			req.register('NotFound', handler);
			return this;
		},
		...anyErrors(req),
		...fetchErrors(req),
		...serverErrors(req),
		...jsonErrors(req),
		...authErrors(req)
	} as unknown as Get;
}
