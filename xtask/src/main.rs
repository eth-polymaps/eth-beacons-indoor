mod api;

use std::fs;
use api::{fetch_beacons, Beacons};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use clap::{Parser, Subcommand};
use regex::Regex;

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
        Command::Generate { uri } => uri,
    };

    let buildings_output = "./src/buildings.gen.rs";
    let beacons_output = "./src/beacons.gen.rs";

    let grouped_beacons = fetch_beacons(&url)?;

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
    println!("Cargo.toml          : ");

    let cargo_toml_path = "./Cargo.toml";
    let content = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

    // Define the feature keys dynamically
    let all: Vec<String> = keys.iter().map(|k| format!("\"{}\"", k)).collect();

    // Generate the new `[features]` section
    let mut new_features_section = format!("[features]\nALL = [{}]\n", all.join(", "));
    for key in &keys {
        new_features_section.push_str(&format!("{} = []\n", key));
    }

    // Corrected regex: Ensures full capture of the existing `[features]` section
    let re = Regex::new(r"(?ms)^\[features\][^\[]*").unwrap();

    let new_content = if re.is_match(&content) {
        // Replace existing `[features]` section
        re.replace(&content, new_features_section).to_string()
    } else {
        // Append `[features]` only if it's missing
        format!("{}\n\n{}", content, new_features_section)
    };

    // Write the modified content back to Cargo.toml
    fs::write(cargo_toml_path, new_content).expect("Failed to write Cargo.toml");

    println!("Updated [features] section in Cargo.toml successfully!");

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
