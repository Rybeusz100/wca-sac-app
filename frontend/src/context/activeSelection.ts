import { Dispatch, SetStateAction, createContext } from "react";

export type ActiveSelection = {
  event: string;
  resultType: string;
  region: string;
};

type ActiveSelectionContextType = {
  activeSelection: ActiveSelection;
  setActiveSelection: Dispatch<SetStateAction<ActiveSelection>>;
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
