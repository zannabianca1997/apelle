import type { SongAddDto, SongQueryDto } from './apis/apelle';

/**
 * Check if a song to be added is the same of a song present in the queue
 * @param addDto the song to add
 * @param song the song present
 */
export function isSameSong(addDto: SongAddDto & { title: string }, song: SongQueryDto): boolean {
	switch (addDto.kind) {
		case 'Youtube':
			if (song.kind === 'Youtube') {
				return addDto.video_id === new URL(song.url!).searchParams.get('v')!;
			}
	}

	return addDto.title === song.name;
}
