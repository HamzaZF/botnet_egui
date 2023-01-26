use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct SystemInfo{
    pub systemInfo : String,
}

impl SystemInfo{
    pub fn getInfos(&self) -> String{
        self.systemInfo.clone()
    }
}