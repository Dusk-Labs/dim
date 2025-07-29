import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { RootState } from "../../store";

export const v1 = createApi({
  reducerPath: "v1",
  baseQuery: fetchBaseQuery({
    baseUrl: `${window.location.protocol}//${window.location.host}/api/v1/`,
    prepareHeaders: (headers, { getState }) => {
      const token = (getState() as RootState).auth.token;

      if (token) {
        headers.set("Authorization", token);
      }

      return headers;
    },
  }),
  endpoints: () => ({}),
});

export default v1;
