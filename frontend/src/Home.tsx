import { useEffect, useRef, useState } from "react";
import { API_URL } from "./constants";
import {
  Button,
  Container,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
} from "@mui/material";
import { useNavigate } from "react-router-dom";

function Home() {
  const navigate = useNavigate();

  const [events, setEvents] = useState({} as Record<string, string>);
  const areEventsLoading = useRef(false);

  const [selectedEvent, setSelectedEvent] = useState("");

  useEffect(() => {
    if (!areEventsLoading.current && !Object.keys(events).length) {
      areEventsLoading.current = true;
      fetch(`${API_URL}/events`)
        .then((res) => res.json())
        .then((data: Record<string, string>) => setEvents(data))
        .finally(() => {
          areEventsLoading.current = false;
        });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function goToGraph() {
    navigate(`/graph/${selectedEvent}_A`);
  }

  return (
    <Container maxWidth="xs">
      <Stack spacing={1} mt={2}>
        <FormControl fullWidth>
          <InputLabel id="event-select-label">Event</InputLabel>
          <Select
            id="event-select"
            labelId="event-select-label"
            label="Event"
            value={selectedEvent}
            onChange={(e) => setSelectedEvent(e.target.value as string)}
          >
            {Object.entries(events).map(([id, name]) => (
              <MenuItem key={id} value={id}>
                {name}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <Button
          disabled={!selectedEvent}
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
