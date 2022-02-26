import v1 from "./";

export type UnmatchedMediaFiles = Record<string, Array<UnmatchedMediaFiles>>;

export interface UnmatchedMediaFile {
  id: number;
  name: string;
  duration?: number;
  target_file: string;
}

export const media = v1.injectEndpoints({
  endpoints: (build) => ({
    getUnmatchedMediaFiles: build.query<UnmatchedMediaFiles, string>({
      query: (id) => `library/${id}/unmatched`,
    }),
  }),
});

export const { useGetUnmatchedMediaFilesQuery } = media;

export default media;
