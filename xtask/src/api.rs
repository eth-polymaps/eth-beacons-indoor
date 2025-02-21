use anyhow::Context;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub(crate) struct BeaconResponse {
    beacons: Vec<Beacon>,
}

impl BeaconResponse {
    pub fn get_beacons(self) -> Beacons {
        let mut grouped_beacons: Beacons = Beacons::new();
        for beacon in self.beacons {
            grouped_beacons
                .entry(beacon.indoor.building.clone())
                .or_default()
                .push(beacon);
        }
        grouped_beacons
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Beacon {
    pub major: u16,
    pub minor: u16,
    pub location: Location,
    pub indoor: Room,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Room {
    pub building: String,
    pub floor: String,
    pub room: String,
}

pub type Beacons = HashMap<String, Vec<Beacon>>;

pub(crate) fn fetch_beacons(url: &str) -> anyhow::Result<Beacons> {
    let response =
        reqwest::blocking::get(url).context(format!("Failed to fetch data from {}", url))?;

    let api_response: BeaconResponse = response.json().context("Failed to read json body")?;

    let mut grouped_beacons = api_response.get_beacons();

    grouped_beacons.insert(
        "SON".to_string(),
        vec![
            Beacon {
                major: 99,
                minor: 16,
                location: Location {
                    lat: 47.539442,
                    lon: 8.293186,
                },
                indoor: Room {
                    building: "SON".to_string(),
                    floor: "A".to_string(),
                    room: "31".to_string(),
                },
            },
            Beacon {
                major: 99,
                minor: 17,
                location: Location {
                    lat: 47.539474,
                    lon: 8.293205,
                },
                indoor: Room {
                    building: "SON".to_string(),
                    floor: "A".to_string(),
                    room: "31".to_string(),
                },
            },
            Beacon {
                major: 99,
                minor: 20,
                location: Location {
                    lat: 47.539487,
                    lon: 8.293172,
                },
                indoor: Room {
                    building: "SON".to_string(),
                    floor: "A".to_string(),
                    room: "31".to_string(),
                },
            },
        ],
    );
    Ok(grouped_beacons)
}
