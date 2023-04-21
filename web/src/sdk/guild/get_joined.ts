import {
    SdkRequest, fetchRequest,
    type FetchErrors, fetchErrors,
    type ServerErrors, serverErrors,
    type JsonErrors, jsonErrors,
    type AuthErrors, authErrors,
    type AnyErrors, anyErrors,
} from "$/sdk/req";
import type { Guild } from "$/sdk/types";

export type GetJoinedResponse = Guild[];

export type GetJoined = FetchErrors<GetJoined> & ServerErrors<GetJoined> & JsonErrors<GetJoined> & AuthErrors<GetJoined> & AnyErrors<SdkRequest<GetJoinedResponse, GetJoinedResponse>>;

export function getJoined(authToken: string) {
    const req = new SdkRequest<GetJoinedResponse, GetJoinedResponse>(() =>
        fetchRequest<GetJoinedResponse>("/guild/get_joined", "GET", {
            authToken,
        })
    );

    return {
        ...anyErrors(req),
        ...fetchErrors(req),
        ...serverErrors(req),
        ...jsonErrors(req),
        ...authErrors(req),
    } as unknown as GetJoined;
};
