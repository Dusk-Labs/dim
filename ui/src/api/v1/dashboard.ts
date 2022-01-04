import v1 from "./index";
import { Media } from "./types";

export interface DashboardData {
  [key: string]: Media[];
}

export const card = v1.injectEndpoints({
  endpoints: (build) => ({
    getCards: build.query<DashboardData, void>({
      query: () => "dashboard"
    })
  })
});

export const { useGetCardsQuery } = card;

export default card;
