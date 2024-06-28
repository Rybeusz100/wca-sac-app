import { Route, BrowserRouter as Router, Routes } from "react-router-dom";
import Home from "./Home";
import Graph from "./Graph";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/graph/:graphType" element={<Graph />} />
      </Routes>
    </Router>
  );
}

export default App;
