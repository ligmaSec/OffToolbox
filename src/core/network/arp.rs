#[derive(PartialEq, serde::Deserialize, serde::Serialize, Debug)]
pub enum ArpModes {
    Passive,
    Active,
}

impl Default for ArpModes {
    fn default() -> ArpModes {
        ArpModes::Active
    }
}

pub fn arp_scan() {
    println!("testt");
}
