pub mod resolver;

include!("buildings.gen.rs");

#[derive(Debug, Clone, Default, Copy)]
pub struct Position {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone)]
pub struct Id {
    pub uuid: &'static str,
    pub major: u16,
    pub minor: u16,
}

#[derive(Debug, Clone)]
pub struct Beacon {
    pub id: Id,
    pub position: Position,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub building: Building,
    pub floor: &'static str,
    pub room: &'static str,
}

pub static ETH_UUID: &str = "58793564-459c-548d-bfcc-367ffd4fcd70";

include!("beacons.gen.rs");
