use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventAnswer(i8);

impl EventAnswer {
    pub fn get_status(&self) -> Result<i8> {
        let status = self.0;
        match status {
            1 => Ok(status),
            _ => Err(Error::EventAnswerUnkownStatus { status }),
        }
    }
}
