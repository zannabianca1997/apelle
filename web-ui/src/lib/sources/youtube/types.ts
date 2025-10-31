
export interface YoutubeThumbnail {
    width: number;
    height: number;
    url: string;
}

export interface YoutubeSearchItemDetails {
    title: string;
    url: string;
    thumbnails: YoutubeThumbnail[];
}

export interface YoutubeSongData {
    video_id: string;
    url: string;
    thumbs: YoutubeThumbnail[];
}