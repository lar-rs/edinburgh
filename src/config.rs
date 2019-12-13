/// Edinburgh sensor config
use serde::{Serialize,Deserialize};



#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Config {
    pub scale_min :  f32,
    pub scale_max :  f32,
}



impl Default for Config {
    fn default() -> Config {
        Config {
            scale_min : 0.0,
            scale_max : 1.0,
        }
    }
}