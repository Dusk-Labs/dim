import { createContext } from "react";

export interface SelectMediaFileContext {
  open: () => void;
  close: () => void;
  currentID?: number | null;
  setClicked: React.Dispatch<React.SetStateAction<boolean>>;
}

// Intentionally naming the variable the same as the type.
// See: https://github.com/typescript-eslint/typescript-eslint/issues/2585
// eslint-disable-next-line @typescript-eslint/no-redeclare
export const SelectMediaFileContext = createContext<SelectMediaFileContext>({
  open: () => {},
  close: () => {},
  currentID: null,
  setClicked: () => {},
});
