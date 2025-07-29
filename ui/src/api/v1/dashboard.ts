import v1 from "./index";
import { Media, Version } from "./types";

/**
 * The results returned by the main dashboard API.
 */
export interface DashboardData {
  [key: string]: Media[];
}

/**
 * The results returned by the dashboard banner API.
 */
export interface DashboardPoster {
  backdrop: string;
  banner_caption: string;
  delta: number;
  duration: number;
  episode: number;
  genres: string[];
  id: number;
  season: number;
  synopsis: string;
  title: string;
  versions: Version[];
  year: number;
}

export const card = v1.injectEndpoints({
  endpoints: (build) => ({
    getBanners: build.query<DashboardPoster[], void>({
      query: () => "dashboard/banner",
    }),
    getCards: build.query<DashboardData, void>({
      query: () => "dashboard",
    }),
  }),
});

export const { useGetBannersQuery, useGetCardsQuery } = card;

export default card;
