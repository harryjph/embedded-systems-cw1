use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bin {
    pub id: u64,
    pub config: BinConfig,
    pub fullness: f64,
    /// If fullness is above this value, the bin is full
    pub full_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinConfig {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub fn dummy_data() -> [Bin; 5] {
    [
        Bin {
            id: 1,
            config: BinConfig {
                name: "Gilbert".to_string(),
                latitude: 51.501,
                longitude: -0.142,
            },
            fullness: 50.0,
            full_threshold: 80.0,
        },
        Bin {
            id: 2,
            config: BinConfig {
                name: "Godfried".to_string(),
                latitude: 51.501,
                longitude: -0.145,
            },
            fullness: 65.0,
            full_threshold: 80.0,
        },
        Bin {
            id: 3,
            config: BinConfig {
                name: "Stephen".to_string(),
                latitude: 51.498,
                longitude: -0.177,
            },
            fullness: 95.0,
            full_threshold: 81.0,
        },
        Bin {
            id: 4,
            config: BinConfig {
                name: "Fry".to_string(),
                latitude: 51.470,
                longitude: -0.454,
            },
            fullness: 85.0,
            full_threshold: 90.0,
        },
        Bin {
            id: 5,
            config: BinConfig {
                name: "Norbert".to_string(),
                latitude: 51.162,
                longitude: -0.177,
            },
            fullness: 99.0,
            full_threshold: 70.0,
        },
    ]
}
