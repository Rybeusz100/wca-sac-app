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
import { Continents, ContinentsContext } from "./context/continents";

function App() {
  const [activeSelection, setActiveSelection] = useState<ActiveSelection>({
    event: "",
    resultType: "",
    region: "",
  });
  const [wcaEvents, setWcaEvents] = useState<WcaEvents>({});
  const areWcaEventsLoading = useRef(false);
  const [continents, setContinents] = useState<Continents>({});
  const areContinentsLoading = useRef(false);

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

    if (!areContinentsLoading.current && !Object.keys(continents).length) {
      areContinentsLoading.current = true;
      fetch(`${API_URL}/continents`)
        .then((res) => res.json())
        .then(setContinents)
        .finally(() => {
          areContinentsLoading.current = false;
        });
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <ActiveSelectionContext.Provider
      value={{ activeSelection, setActiveSelection }}
    >
      <WcaEventsContext.Provider value={wcaEvents}>
        <ContinentsContext.Provider value={continents}>
          <Router>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/graph/:graphType" element={<Graph />} />
            </Routes>
          </Router>
        </ContinentsContext.Provider>
      </WcaEventsContext.Provider>
    </ActiveSelectionContext.Provider>
  );
}

export default App;
