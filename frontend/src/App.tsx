import { Route, BrowserRouter as Router, Routes } from "react-router-dom";
import Home from "./Home";
import Graph from "./Graph";
import { useEffect, useRef, useState } from "react";
import {
  ActiveSelectionContext,
  ActiveSelection,
} from "./context/activeSelection";
import { WcaEvents, WcaEventsContext } from "./context/wcaEvents";
import { API_URL } from "./constants";

function App() {
  const [activeSelection, setActiveSelection] = useState<ActiveSelection>({
    event: "",
    resultType: "",
    region: "",
  });
  const [wcaEvents, setWcaEvents] = useState({} as WcaEvents);
  const areWcaEventsLoading = useRef(false);

  useEffect(() => {
    if (!areWcaEventsLoading.current && !Object.keys(wcaEvents).length) {
      areWcaEventsLoading.current = true;
      fetch(`${API_URL}/events`)
        .then((res) => res.json())
        .then(setWcaEvents)
        .finally(() => {
          areWcaEventsLoading.current = false;
        });
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <ActiveSelectionContext.Provider
      value={{ activeSelection, setActiveSelection }}
    >
      <WcaEventsContext.Provider value={wcaEvents}>
        <Router>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/graph/:graphType" element={<Graph />} />
          </Routes>
        </Router>
      </WcaEventsContext.Provider>
    </ActiveSelectionContext.Provider>
  );
}

export default App;
