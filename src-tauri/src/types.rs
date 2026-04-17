use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)] //Makro für z.b Json in die Struct atomatisch parsen
pub struct Position {
    pub lat: f64,
    pub lon: f64,
}
