import v1 from "./";

export const mediafile = v1.injectEndpoints({
  endpoints: (build) => ({
    matchMediafiles: build.query<
      null,
      { ids: Array<number>; external_id: number; media_type: string }
    >({
      query: ({ ids, external_id, media_type }) => ({
        url: `mediafile/match`,
        body: {
          mediafiles: ids,
          tmdb_id: external_id,
          media_type: media_type,
        },
        method: `PATCH`,
      }),
    }),
  }),
});

export const { useMatchMediafilesQuery } = mediafile;

export default mediafile;
