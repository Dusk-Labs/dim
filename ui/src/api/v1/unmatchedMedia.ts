import v1 from "./";

export interface UnmatchedMediaFiles {
  count: number;
  files: Array<UnmatchedMediaFile>;
}

export interface UnmatchedMediaFile {
  id?: number;
  name?: string;
  folder?: string;
  duration?: number;
  file?: string;
  files?: Array<UnmatchedMediaFile>;

  // If the type is "directory", only the `files` and `folder` fields will be accessible.
  // Otherwise, all the other fields will be accessible, with `duration` remaining optional.
  type: "directory" | "file";
}

export const media = v1.injectEndpoints({
  endpoints: (build) => ({
    getUnmatchedMediaFiles: build.query<
      UnmatchedMediaFiles,
      { id: string; search?: string | null }
    >({
      query: ({ id, search }) => {
        if (search && search.length > 0)
          return `library/${id}/unmatched?search=${search}`;

        return `library/${id}/unmatched`;
      },
    }),
  }),
});

export const { useGetUnmatchedMediaFilesQuery } = media;

export default media;
