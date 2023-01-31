use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: u64,
    name: String,
    latitude: f64,
    longitude: f64,
    fullness: f64,
    /// If fullness is above this value, the bin is full
    full_threshold: f64,
}

pub fn dummy_data() -> [Node; 5] {
    [
        Node {
            id: 1,
            name: "Gilbert".to_string(),
            latitude: 51.501,
            longitude: -0.142,
            fullness: 50.0,
            full_threshold: 80.0,
        },
        Node {
            id: 2,
            name: "Godfried".to_string(),
            latitude: 51.501,
            longitude: -0.145,
            fullness: 65.0,
            full_threshold: 80.0,
        },
        Node {
            id: 3,
            name: "Stephen".to_string(),
            latitude: 51.498,
            longitude: -0.177,
            fullness: 95.0,
            full_threshold: 81.0,
        },
        Node {
            id: 4,
            name: "Fry".to_string(),
            latitude: 51.470,
            longitude: -0.454,
            fullness: 85.0,
            full_threshold: 90.0,
        },
        Node {
            id: 5,
            name: "Gilbert".to_string(),
            latitude: 51.162,
            longitude: -0.177,
            fullness: 99.0,
            full_threshold: 70.0,
        },
    ]
}
