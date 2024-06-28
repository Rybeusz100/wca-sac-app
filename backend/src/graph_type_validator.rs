use std::{collections::HashMap, fs};

pub struct GraphTypeValidator {
    events: HashMap<String, String>,
    continents: HashMap<String, String>,
    countries: HashMap<String, String>,
}

impl GraphTypeValidator {
    pub fn new() -> Self {
        // Events
        let events_data = fs::read_to_string("./assets/events.json").unwrap();
        let events = serde_json::from_str(&events_data).unwrap();

        // Continents
        let continents_data = fs::read_to_string("./assets/continents.json").unwrap();
        let continents = serde_json::from_str(&continents_data).unwrap();

        // Countries
        let countries_data =
            fs::read_to_string("../WCA_SAC/data/WCA_export_Countries.tsv").unwrap();
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(countries_data.as_bytes());
        let mut countries = HashMap::new();
        for result in reader.records() {
            let record = result.unwrap();
            countries.insert(record[2].to_owned(), record[3].to_owned());
        }

        Self {
            events,
            continents,
            countries,
        }
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

        // Region - optional
        if let Some(region) = graph_type_segments.next() {
            if !self.continents.contains_key(region) && !self.countries.contains_key(region) {
                return false;
            }
        }

        true
    }

    pub fn countries(&self) -> &HashMap<String, String> {
        &self.countries
    }
}
