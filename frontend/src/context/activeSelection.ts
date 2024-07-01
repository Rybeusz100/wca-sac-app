import { Dispatch, SetStateAction, createContext } from "react";

export type ActiveSelectionType = {
  event: string;
  resultType: string;
  region: string;
};

type ActiveSelectionContextType = {
  activeSelection: ActiveSelectionType;
  setActiveSelection: Dispatch<SetStateAction<ActiveSelectionType>>;
};

export const ActiveSelectionContext = createContext<ActiveSelectionContextType>(
  {
    activeSelection: {
      event: "",
      resultType: "",
      region: "",
    },
    setActiveSelection: () => {},
  }
);
