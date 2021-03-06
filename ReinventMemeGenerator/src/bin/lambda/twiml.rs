// Taken from https://github.com/neil-lobracco/twilio-rs

pub trait Action {
    fn as_twiml(&self) -> String;
}

pub struct Twiml {
    body: String,
}

impl Twiml {
    pub fn new() -> Twiml {
        Twiml {
            body: "".to_string(),
        }
    }

    pub fn add(&mut self, a: &dyn Action) -> &mut Twiml {
        let twiml = a.as_twiml();
        self.body.push_str((&twiml as &dyn AsRef<str>).as_ref());
        self
    }

    pub fn as_twiml(&self) -> String {
        let b: &str = self.body.as_ref();
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" ?><Response>{}</Response>",
            b
        )
    }
}

pub fn format_xml_string(tag: &str, attributes: &[(&str, &str)], inner: &str) -> String {
    let attribute_string = match attributes.len() {
        0 => "".to_string(),
        _ => attributes
            .iter()
            .map(|t| format!("{}=\"{}\"", t.0, t.1))
            .fold("".to_string(), |mut acc, v| {
                acc.push(' ');
                acc.push_str(&v);
                acc
            }),
    };
    let attribute_str: &str = attribute_string.as_ref();
    format!("<{}{}>{}</{}>", tag, attribute_str, inner, tag)
}
