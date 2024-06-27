import { useEffect, useRef, useState } from "react";
import { API_URL } from "./constants";
import { Box, FormControl, InputLabel, MenuItem, Select } from "@mui/material";
import Viewer from "react-viewer";

function App() {
  const [events, setEvents] = useState({} as Record<string, string>);
  const areEventsLoading = useRef(false);

  const [selectedEvent, setSelectedEvent] = useState("");

  useEffect(() => {
    if (!areEventsLoading.current && !Object.keys(events).length) {
      areEventsLoading.current = true;
      fetch(`${API_URL}/events`)
        .then((res) => res.json())
        .then((data: Record<string, string>) => {
          setEvents(data);
        })
        .finally(() => {
          areEventsLoading.current = false;
        });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <>
      <FormControl
        style={{
          width: 100,
        }}
      >
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
      <Box id="viewer-container">
        <Viewer
          container={document.getElementById("viewer-container")!}
          visible={true}
          images={[
            {
              src: `${API_URL}/graph/${selectedEvent}_A`,
            },
          ]}
        />
      </Box>
    </>
  );
}

export default App;
