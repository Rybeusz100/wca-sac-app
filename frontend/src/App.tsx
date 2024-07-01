import { Route, BrowserRouter as Router, Routes } from "react-router-dom";
import Home from "./Home";
import Graph from "./Graph";
import { useState } from "react";
import {
  ActiveSelectionContext,
  ActiveSelectionType,
} from "./context/activeSelection";

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
