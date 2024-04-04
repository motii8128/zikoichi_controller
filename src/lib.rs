use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Wheel{
    pub  front_left:f64,
    pub  front_right:f64,
    pub  rear_left:f64,
    pub  rear_right:f64,
}

impl Wheel {
    pub fn new(fl:f64, fr:f64, rl:f64, rr:f64)->Wheel
    {
        Wheel{front_left:fl, front_right:fr, rear_left:rl, rear_right:rr}
    }
    pub fn serialize(&self)->String
    {
        serde_json::to_string(&self).unwrap()
    }
    pub fn deserialize(str_value:String)->Wheel
    {
        let result:Wheel = serde_json::from_str(&str_value).unwrap();
        result
    }
}