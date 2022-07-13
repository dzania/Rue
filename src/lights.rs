#![allow(dead_code)]

pub struct LightState {
    pub on: bool,
    pub bri: u8,
    pub hue: u16,
    pub sat: u8,
    pub xy: [f64; 2],
    pub ct: u16,
    pub alert: String,
    pub effect: String,
    pub color_mode: String,
    pub reachable: bool,
}
pub struct Light {
    pub name: String,
    pub state: LightState,
    pub modelid: String,
    pub swversion: String,
    pub unique_id: String,
}

pub enum GroupType {
    Luminaire,
    LightSource,
    LightGroup,
    Room,
    Entertainment,
    Zone,
}

pub struct GroupState {
    any_on: bool,
    all_on: bool,
}

pub struct GroupAction {
    on: bool,
    hue: u16,
    effect: String,
    bri: u8,
    sat: u8,
    ct: u16,
    xy: [f64; 2],
}

pub struct Group {
    pub name: String,
    pub lights: Vec<usize>,
    pub state: Option<GroupState>,
    pub r#type: GroupType,
    pub action: GroupAction,
}
