use std::{collections::HashMap, fs};

pub struct GraphTypeValidator {
    events: HashMap<String, String>,
    continents: HashMap<String, String>,
}

impl GraphTypeValidator {
    pub fn new() -> Self {
        let events_data = fs::read_to_string("./assets/events.json").unwrap();
        let events = serde_json::from_str(&events_data).unwrap();

        let continents_data = fs::read_to_string("./assets/continents.json").unwrap();
        let continents = serde_json::from_str(&continents_data).unwrap();

        Self { events, continents }
    }

    pub fn validate(&self, graph_type: &str) -> bool {
        let mut graph_type_segments = graph_type.split('_');

        // Event - required
        let event = graph_type_segments.next().unwrap_or_default();
        if !self.events.contains_key(event) {
            return false;
        }

        // Result type - required
        let result_type = graph_type_segments.next().unwrap_or_default();
        if result_type != "S" && result_type != "A" {
            return false;
        }

        // Continent - optional
        if let Some(continent) = graph_type_segments.next() {
            if !self.continents.contains_key(continent) {
                return false;
            }
        }

        true
    }
}
