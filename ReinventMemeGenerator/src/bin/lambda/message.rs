// Taken from https://github.com/neil-lobracco/twilio-rs

use crate::twiml::{format_xml_string, Action};

pub struct Message {
    pub txt: String,
}

impl Action for Message {
    fn as_twiml(&self) -> String {
        format_xml_string("Message", &Vec::new(), &self.txt)
    }
}
