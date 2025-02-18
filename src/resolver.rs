use crate::Beacon;

pub fn find_beacon_by_id(uuid: &str, major: u16, minor: u16) -> Option<&Beacon> {
    super::BEACONS
        .iter()
        .find(|x| uuid == x.id.uuid && major == x.id.major && minor == x.id.minor)
}
