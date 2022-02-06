import React, { createContext } from "react";

interface ApiEpisode {
  id: number;
  name?: string | null;
  overview?: string | null;
  episode?: number | null;
  still?: string | null;
  still_file?: string | null;
}

interface ApiSeason {
  id: number;
  name?: string | null;
  poster_path?: string | null;
  poster_file?: string | null;
  season_number: number;
  episodes: ApiEpisode[];
}

interface ApiMedia {
  id: number;
  title: string;
  release_date?: string | null;
  overview?: string | null;
  poster_path?: string | null;
  backdrop_path?: string | null;
  poster_file?: string | null;
  backdrop_file?: string | null;
  genres: string[];
  rating?: number | null;
  seasons: ApiSeason[];
}

interface RematchContext {
  mediaType: string;
  setMediaType: React.Dispatch<React.SetStateAction<string>>;
  tmdbResults: ApiMedia[];
  setTmdbResults: React.Dispatch<React.SetStateAction<ApiMedia[]>>;
  query: string;
  setQuery: React.Dispatch<React.SetStateAction<string>>;
  tmdbID: number | null;
  setTmdbID: React.Dispatch<React.SetStateAction<number | null>>;
}

// Placeholder to allow an initial RematchContext value to be created.
const nullFn = () => {};

// Intentionally naming the variable the same as the type.
// See: https://github.com/typescript-eslint/typescript-eslint/issues/2585
// eslint-disable-next-line @typescript-eslint/no-redeclare
export const RematchContext = createContext<RematchContext>({
  mediaType: "",
  setMediaType: nullFn,
  tmdbResults: [],
  setTmdbResults: nullFn,
  query: "",
  setQuery: nullFn,
  tmdbID: null,
  setTmdbID: nullFn,
});
