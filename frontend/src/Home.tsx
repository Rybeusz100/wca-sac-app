import { useContext, useEffect, useRef, useState } from "react";
import { API_URL } from "./constants";
import {
  Button,
  Container,
  FormControl,
  InputLabel,
  ListSubheader,
  MenuItem,
  Select,
  Stack,
} from "@mui/material";
import { useNavigate } from "react-router-dom";
import { ActiveSelectionContext } from "./App";

function Home() {
  const navigate = useNavigate();
  const { activeSelection, setActiveSelection } = useContext(
    ActiveSelectionContext
  );

  const [events, setEvents] = useState({} as Record<string, string>);
  const areEventsLoading = useRef(false);

  const [continents, setContinents] = useState({} as Record<string, string>);
  const areContinentsLoading = useRef(false);

  const [countries, setCountries] = useState({} as Record<string, string>);
  const areCountriesLoading = useRef(false);

  useEffect(() => {
    if (!areEventsLoading.current && !Object.keys(events).length) {
      areEventsLoading.current = true;
      fetch(`${API_URL}/events`)
        .then((res) => res.json())
        .then(setEvents)
        .finally(() => {
          areEventsLoading.current = false;
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

    if (!areCountriesLoading.current && !Object.keys(countries).length) {
      areCountriesLoading.current = true;
      fetch(`${API_URL}/countries`)
        .then((res) => res.json())
        .then(setCountries)
        .finally(() => {
          areCountriesLoading.current = false;
        });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function goToGraph() {
    let path = `/graph/${activeSelection.event}_${activeSelection.resultType}`;

    if (activeSelection.region) {
      path += `_${activeSelection.region}`;
    }

    navigate(path);
  }

  return (
    <Container maxWidth="xs">
      <Stack spacing={2} mt={2}>
        <FormControl disabled={!Object.keys(events).length}>
          <InputLabel id="event-select-label">Event</InputLabel>
          <Select
            id="event-select"
            labelId="event-select-label"
            label="Event"
            value={activeSelection.event}
            onChange={(e) =>
              setActiveSelection((prev) => ({
                ...prev,
                event: e.target.value,
              }))
            }
          >
            {Object.entries(events).map(([id, name]) => (
              <MenuItem key={id} value={id}>
                {name}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <FormControl>
          <InputLabel id="result-type-select-label">Result type</InputLabel>
          <Select
            id="result-type-select"
            labelId="result-type-select-label"
            label="Result type"
            value={activeSelection.resultType}
            onChange={(e) =>
              setActiveSelection((prev) => ({
                ...prev,
                resultType: e.target.value,
              }))
            }
          >
            <MenuItem value="S">Single</MenuItem>
            <MenuItem value="A">Average</MenuItem>
          </Select>
        </FormControl>
        <FormControl
          disabled={
            !Object.keys(continents).length && !Object.keys(countries).length
          }
        >
          <InputLabel id="region-select-label">Region (optional)</InputLabel>
          <Select
            id="region-select"
            labelId="region-select-label"
            label="Region (optional)"
            value={activeSelection.region}
            onChange={(e) =>
              setActiveSelection((prev) => ({
                ...prev,
                region: e.target.value,
              }))
            }
          >
            <ListSubheader>Continent</ListSubheader>
            {Object.entries(continents)
              .sort((a, b) => a[1].localeCompare(b[1]))
              .map(([id, name]) => (
                <MenuItem key={id} value={id}>
                  {name}
                </MenuItem>
              ))}

            <ListSubheader>Country</ListSubheader>
            {Object.entries(countries)
              .sort((a, b) => a[1].localeCompare(b[1]))
              .map(([id, name]) => (
                <MenuItem key={id} value={id}>
                  {name}
                </MenuItem>
              ))}
          </Select>
        </FormControl>

        <Button
          disabled={!activeSelection.event || !activeSelection.resultType}
          onClick={goToGraph}
          variant="contained"
        >
          Generate graph
        </Button>
      </Stack>
    </Container>
  );
}

export default Home;
