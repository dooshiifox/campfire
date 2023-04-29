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
import type { Handler, Snowflake, User } from '$/sdk/types';

export type LoginBody = {
	email: string;
	password: string;
};

export type LoginResponse = {
	access_token: string;
	user: User;
};

export type Login = {
	onInvalidCredentials(handler: Handler): Login;
} & FetchErrors<Login> &
	ServerErrors<Login> &
	JsonErrors<Login> &
	AuthErrors<Login> &
	AnyErrors<SdkRequest<LoginResponse, LoginResponse>>;

export function login(body: LoginBody) {
	const req = new SdkRequest<LoginResponse, LoginResponse>(() =>
		fetchRequest<LoginResponse>('/account/login', 'POST', {
			body
		})
	);

	return {
		onInvalidCredentials(handler: Handler) {
			req.register('InvalidCredentials', handler);
			return this;
		},
		...anyErrors(req),
		...fetchErrors(req),
		...serverErrors(req),
		...jsonErrors(req),
		...authErrors(req)
	} as unknown as Login;
}
