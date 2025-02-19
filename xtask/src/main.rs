use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    beacons: Vec<Beacon>,
}

#[derive(Deserialize, Debug)]
struct Beacon {
    major: u16,
    minor: u16,
    location: Location,
    indoor: Room,
}

#[derive(Deserialize, Debug)]
struct Location {
    lat: f64,
    lon: f64,
}

#[derive(Deserialize, Debug)]
struct Room {
    building: String,
    floor: String,
    room: String,
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Generate {
        #[arg(short, long)]
        uri: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let url = match args.command {
        Command::Generate { uri: url } => url,
    };

    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch data from {}: {}", url, e))?;

    let api_response: ApiResponse = response
        .json()
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    let mut grouped_beacons: HashMap<String, Vec<Beacon>> = HashMap::new();
    for beacon in api_response.beacons {
        grouped_beacons
            .entry(beacon.indoor.building.clone())
            .or_insert_with(Vec::new)
            .push(beacon);
    }

    let output_path = Path::new("./src/generated.rs");
    let file = File::create(&output_path)?;
    let mut writer = BufWriter::new(&file);

    writeln!(
        writer,
        r#"
pub mod resolver;

#[derive(Debug, Clone, Default, Copy)]
pub struct Position {{
    pub lat: f64,
    pub lon: f64,
}}

#[derive(Debug, Clone)]
pub struct Id {{
    pub uuid:  &'static str,
    pub major: u16,
    pub minor: u16,
}}

#[derive(Debug, Clone)]
pub struct Beacon {{
    pub id: Id,
    pub position: Position,
    pub location: Location,
}}

#[derive(Debug, Clone)]
pub struct Location {{
    pub building: &'static str,
    pub floor: &'static str,
    pub room: &'static str,
}}

pub static ETH_UUID: &str = "58793564-459c-548d-bfcc-367ffd4fcd70";

pub static BEACONS: &[Beacon] = &[

    #[cfg(feature = "SON")]
    Beacon {{
        id: Id {{
            uuid: ETH_UUID,
            major: 99,
            minor: 16,
        }},
        position: Position {{
            lat: 47.539442,
            lon: 8.293186,
        }},
        location: Location {{
            building: "SON",
            floor: "A",
            room: "31",
        }},
    }},
    #[cfg(feature = "SON")]
    Beacon {{
        id: Id {{
            uuid: ETH_UUID,
            major: 99,
            minor: 17,
        }},
        position: Position {{
            lat: 47.539474,
            lon: 8.293205,
        }},
        location: Location {{
            building: "SON",
            floor: "A",
            room: "31",
        }},
    }},
    #[cfg(feature = "SON")]
    Beacon {{
        id: Id {{
            uuid: ETH_UUID,
            major: 99,
            minor: 20,
        }},
        position: Position {{
            lat: 47.539487,
            lon: 8.293172,
        }},
            location: Location {{
            building: "SON",
            floor: "A",
            room: "31",
        }},
    }},
"#
    )?;

    let mut keys: Vec<String> = grouped_beacons.keys().cloned().collect();
    keys.sort();

    for feature_flag in &keys {
        let beacons = grouped_beacons.get(&feature_flag.clone()).unwrap();
        for beacon in beacons {
            writeln!(writer, "    #[cfg(feature = \"{}\")]", feature_flag)?;
            writeln!(
                writer,
                r#"    Beacon {{
        id: Id {{ uuid: ETH_UUID, major: {}, minor: {} }},
        position: Position {{ lat: {}, lon: {} }},
        location: Location {{ building: "{}", floor: "{}", room: "{}" }},
    }},
"#,
                beacon.major,
                beacon.minor,
                beacon.location.lat,
                beacon.location.lon,
                beacon.indoor.building,
                beacon.indoor.floor,
                beacon.indoor.room
            )?;
        }
    }
    writeln!(writer, "];")?;

    println!("----------------------");
    println!("features");
    for key in keys {
        println!("{} = []", key);
    }

    println!("Generated file: {:?}", output_path);
    Ok(())
}
