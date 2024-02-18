#[derive(PartialEq)]
pub enum ArpModes {
    Passive,
    Active,
}

impl ArpModes {
    pub fn default() -> ArpModes {
        ArpModes::Active
    }
}

pub fn arp_scan() {
    println!("testt");
}
