import type { Type } from '$/sdk';

type ImgSize = 16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048;

export function guildIcon(guild: Type.Guild, size: ImgSize = 64): string {
	return '';
	// return `http://localhost:8080/cdn/`;
}

export function profileImage(user?: Type.User, size: ImgSize = 64): string {
	return '';
	// return `http://localhost:8080/cdn/`;
}
