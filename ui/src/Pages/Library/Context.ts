import React, { createContext } from "react";

interface UnmatchedMedia {
  id: number;
  name: string;
  duration?: number | null;
  target_file: string;
}

interface DimError {
  error: string;
  message: string;
}

interface LibraryUnmatched {
  items: Record<string, UnmatchedMedia>;
  fetching: boolean;
  fetched: boolean;
  error: DimError | null;
}

interface LibraryContext {
  setShowUnmatched: React.Dispatch<React.SetStateAction<boolean>>;
  showUnmatched: boolean;
  unmatched: LibraryUnmatched;
}

// Intentionally naming the variable the same as the type.
// See: https://github.com/typescript-eslint/typescript-eslint/issues/2585
// eslint-disable-next-line @typescript-eslint/no-redeclare
export const LibraryContext = createContext<LibraryContext | null>(null);
