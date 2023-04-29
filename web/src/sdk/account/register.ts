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
import type { Handler, User } from '$/sdk/types';

export type RegisterBody = {
	username: string;
	email: string;
	password: string;
};

export type RegisterResponse = {
	access_token: string;
	user: User;
};

export type Register = {
	onInvalidUsername(handler: Handler): Register;
	onUsernameTaken(handler: Handler): Register;
	onInvalidEmail(handler: Handler): Register;
	onEmailTaken(handler: Handler): Register;
	onPasswordTooWeak(handler: Handler): Register;
	onPasswordTooLong(handler: Handler): Register;
	onPasswordTooShort(handler: Handler): Register;
	onPasswordTooCommon(handler: Handler): Register;
	onPasswordLikeUsername(handler: Handler): Register;
	onPasswordLikeEmail(handler: Handler): Register;
} & FetchErrors<Register> &
	ServerErrors<Register> &
	JsonErrors<Register> &
	AuthErrors<Register> &
	AnyErrors<SdkRequest<RegisterResponse, RegisterResponse>>;

export function register(body: RegisterBody) {
	const req = new SdkRequest<RegisterResponse, RegisterResponse>(() =>
		fetchRequest<RegisterResponse>('/account/register', 'POST', {
			body
		})
	);

	return {
		onInvalidUsername(handler: Handler) {
			req.register('InvalidUsername', handler);
			return this;
		},
		onUsernameTaken(handler: Handler) {
			req.register('UsernameTaken', handler);
			return this;
		},
		onInvalidEmail(handler: Handler) {
			req.register('InvalidEmail', handler);
			return this;
		},
		onEmailTaken(handler: Handler) {
			req.register('EmailTaken', handler);
			return this;
		},
		onPasswordTooWeak(handler: Handler) {
			req.register('PasswordTooWeak', handler);
			return this;
		},
		onPasswordTooLong(handler: Handler) {
			req.register('PasswordTooLong', handler);
			return this;
		},
		onPasswordTooShort(handler: Handler) {
			req.register('PasswordTooShort', handler);
			return this;
		},
		onPasswordTooCommon(handler: Handler) {
			req.register('PasswordTooCommon', handler);
			return this;
		},
		onPasswordLikeUsername(handler: Handler) {
			req.register('PasswordLikeUsername', handler);
			return this;
		},
		onPasswordLikeEmail(handler: Handler) {
			req.register('PasswordLikeEmail', handler);
			return this;
		},
		...anyErrors(req),
		...fetchErrors(req),
		...serverErrors(req),
		...jsonErrors(req),
		...authErrors(req)
	} as unknown as Register;
}
