use serde::{Deserialize, Serialize};

// use crate::error::*;
use super::Scale;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    AIDE50_150_500,
    AIDE100_1000_3000,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Aide{
    pub model: Model,
    pub scale: Vec<Scale>,
}

impl Default for Aide {
    fn default() -> Aide {
        Aide {
            model: Model::AIDE50_150_500,
            scale: vec![ Scale::new(1.0), Scale::new(1.0), Scale::new(1.0)],
        }
    }
}

