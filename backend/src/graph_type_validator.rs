use std::{collections::HashMap, fs};

pub struct GraphTypeValidator {
    events: HashMap<String, String>,
}

impl GraphTypeValidator {
    pub fn new() -> Self {
        let events_data = fs::read_to_string("./assets/events.json").unwrap();
        let events = serde_json::from_str(&events_data).unwrap();

        Self { events }
    }

    pub fn validate(&self, graph_type: &str) -> bool {
        let mut graph_type_segments = graph_type.split('_');
        let event = graph_type_segments.next().unwrap_or_default();
        if !self.events.contains_key(event) {
            return false;
        }

        let result_type = graph_type_segments.next().unwrap_or_default();
        if result_type != "S" && result_type != "A" {
            return false;
        }

        true
    }
}
