import v1 from "./";

import type { Media } from "./types";

/**
 * A file associated with a piece of media.
 */
export interface MediaFile {
  audio: string;
  audio_language: null;
  channels: number;
  codec: string;
  container: string;
  corrupt: boolean;
  duration: number;
  episode: null;
  id: number;
  library_id: number;
  media_id: number;
  original_resolution: null;
  profile: string;
  quality: string;
  raw_name: string;
  raw_year: number;
  season: null;
  target_file: string;
}

/**
 * A season of a TV show.
 */
export interface Season {
  added: string;
  id: number;
  poster: string;
  season_number: number;
  tvshowid: number;
}

/**
 * An episode of a TV show.
 */
export interface Episode {
  episode: number;
  id: number;
  name: number;
  thumbnail_url: string | null;
}

export const media = v1.injectEndpoints({
  endpoints: (build) => ({
    getMediaEpisodes: build.query<Episode[], string>({
      query: (id) => `season/${id}/episodes`,
    }),
    getMediaFiles: build.query<MediaFile[], string>({
      query: (id) => `media/${id}/files`,
    }),
    getMedia: build.query<Media, string>({
      query: (id) => `media/${id}`,
    }),
    getMediaSeasons: build.query<Season[], string>({
      query: (id) => `tv/${id}/season`,
    }),
  }),
});

export const {
  useGetMediaEpisodesQuery,
  useGetMediaFilesQuery,
  useGetMediaQuery,
  useGetMediaSeasonsQuery,
} = media;

export default media;
