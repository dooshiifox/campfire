/////////////////////////////
//   SERVER RESPONSE TYPES
/////////////////////////////

export type ApiResponse<T> = ApiResponseSuccess<T> | ApiResponseError<string>;
export type ApiResponseSuccess<T> = {
    error: false;
    data: T;
};
export type ApiResponseError<M extends string, E = unknown> = {
    error: true;
    message: M;
    data: E;
};

/////////////////////////////
//       SDK TYPES
/////////////////////////////

export type Handler<Args extends unknown[] = []> = (
    msg: string,
    ...args: Args
    ) => void;

export class UnhandledReqError extends Error {}

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
    discriminator: number;
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