import { Route, BrowserRouter as Router, Routes } from "react-router-dom";
import Home from "./Home";
import Graph from "./Graph";
import { Dispatch, SetStateAction, createContext, useState } from "react";

type ActiveSelectionType = {
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

function App() {
  const [activeSelection, setActiveSelection] = useState<ActiveSelectionType>({
    event: "",
    resultType: "",
    region: "",
  });

  return (
    <ActiveSelectionContext.Provider
      value={{ activeSelection, setActiveSelection }}
    >
      <Router>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/graph/:graphType" element={<Graph />} />
        </Routes>
      </Router>
    </ActiveSelectionContext.Provider>
  );
}

export default App;
