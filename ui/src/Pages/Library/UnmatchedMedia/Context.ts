import React, { createContext } from "react";

interface SelectedFile {
  id: string;
  name: string | undefined;
  parent: string;
}

interface SelectedFiles {
  [fileId: string]: SelectedFile;
}

interface Media {
  id: number;
  title: string;
  poster_path?: string | undefined;
}

interface SelectUnmatchedContext {
  selectedFiles: SelectedFiles;
  setSelectedFiles: React.Dispatch<React.SetStateAction<SelectedFiles>>;
  currentFolder: string | undefined;
  setCurrentFolder: React.Dispatch<React.SetStateAction<string | undefined>>;
  mediaType: string | undefined;
  setMediaType: React.Dispatch<React.SetStateAction<string | undefined>>;
  tmdbResults: Media[];
  setTmdbResults: React.Dispatch<React.SetStateAction<Media[]>>;
  query: string;
  setQuery: React.Dispatch<React.SetStateAction<string>>;
  tmdbID: number | undefined;
  setTmdbID: React.Dispatch<React.SetStateAction<number | undefined>>;
  filesMatched: string[];
  setFilesMatched: React.Dispatch<React.SetStateAction<string[]>>;
  matching: boolean;
  setMatching: React.Dispatch<React.SetStateAction<boolean>>;
  clearData: () => void;
}

// Placeholder to allow an initial SelectUnmatchedContext value to be created.
const nullFn = () => {};

// Intentionally naming the variable the same as the type.
// See: https://github.com/typescript-eslint/typescript-eslint/issues/2585
// eslint-disable-next-line @typescript-eslint/no-redeclare
export const SelectUnmatchedContext =
  createContext<SelectUnmatchedContext | null>({
    selectedFiles: {},
    setSelectedFiles: nullFn,
    currentFolder: undefined,
    setCurrentFolder: nullFn,
    mediaType: undefined,
    setMediaType: nullFn,
    tmdbResults: [],
    setTmdbResults: nullFn,
    query: "",
    setQuery: nullFn,
    tmdbID: undefined,
    setTmdbID: nullFn,
    filesMatched: [],
    setFilesMatched: nullFn,
    matching: false,
    setMatching: nullFn,
    clearData: nullFn,
  });
