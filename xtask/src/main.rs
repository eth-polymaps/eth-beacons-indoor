use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    beacons: Vec<Beacon>,
}

impl ApiResponse {
    fn get_beacons(self) -> Beacons {
        let mut grouped_beacons: Beacons = Beacons::new();
        for beacon in self.beacons {
            grouped_beacons
                .entry(beacon.indoor.building.clone())
                .or_insert_with(Vec::new)
                .push(beacon);
        }
        grouped_beacons
    }
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

type Beacons = HashMap<String, Vec<Beacon>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let url = match args.command {
        Command::Generate { uri } => uri,
    };

    let buildings_output = "./src/buildings.gen.rs";
    let beacons_output = "./src/beacons.gen.rs";

    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch data from {}: {}", url, e))?;

    let api_response: ApiResponse = response
        .json()
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    let mut grouped_beacons = api_response.get_beacons();

    grouped_beacons.insert("SON".to_string(), vec![
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
    ]);

    let mut writer = BufWriter::new(File::create(Path::new(&beacons_output))?);
    write_beacons(&mut writer, &grouped_beacons)?;

    let mut buildings_writer = BufWriter::new(File::create(Path::new(&buildings_output))?);
    write_buildings(
        &mut buildings_writer,
        grouped_beacons.keys().cloned().collect(),
    )?;

    let mut keys: Vec<String> = grouped_beacons.keys().cloned().collect();
    keys.sort();

    println!("Beacons file        : {:?}", beacons_output);
    println!("Features (buildings): {}", keys.join(", "));

    Ok(())
}

fn write_buildings(writer: &mut BufWriter<File>, mut buildings: Vec<String>) -> anyhow::Result<()> {
    buildings.sort();

    writeln!(
        writer,
        r#"
use strum_macros::AsRefStr;

#[derive(Debug, Clone, AsRefStr)]
pub enum Building {{"#
    )?;

    for building in buildings {
        writeln!(writer, "    {},", building)?;
    }

    writeln!(
        writer,
        r#"
}}
"#
    )?;

    Ok(())
}

fn write_beacons(writer: &mut dyn Write, grouped_beacons: &Beacons) -> anyhow::Result<()> {
    writeln!(
        writer,
        r#"

pub static BEACONS: &[Beacon] = &["#
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
        location: Location {{ building: Building::{}, floor: "{}", room: "{}" }},
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
    Ok(())
}
