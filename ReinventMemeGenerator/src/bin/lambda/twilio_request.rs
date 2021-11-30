#[derive(Default)]
pub struct TwilioRequestBuilder {
    pub to_state: Option<String>,
    pub to_country: Option<String>,
    pub sms_message_sid: Option<String>,
    pub num_media: Option<usize>,
    pub to_city: Option<String>,
    pub from_zip: Option<String>,
    pub sms_sid: Option<String>,
    pub from_state: Option<String>,
    pub sms_status: Option<String>,
    pub from_city: Option<String>,
    pub body: Option<String>,
    pub from_country: Option<String>,
    pub to: Option<String>,
    pub to_zip: Option<()>,
    pub num_segments: Option<usize>,
    pub message_sid: Option<String>,
    pub account_sid: Option<String>,
    pub from: Option<String>,
    pub api_version: Option<String>,
    pub media_urls: Option<Vec<String>>,
    pub media_content_types: Option<Vec<String>>,
}

impl TwilioRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> TwilioRequest {
        TwilioRequest {
            to_state: self.to_state.expect("missing 'to_state' field"),
            to_country: self.to_country.expect("missing 'to_country' field"),
            sms_message_sid: self
                .sms_message_sid
                .expect("missing 'sms_message_sid' field"),
            num_media: self.num_media.expect("missing 'num_media' field"),
            to_city: self.to_city.expect("missing 'to_city' field"),
            from_zip: self.from_zip.expect("missing 'from_zip' field"),
            sms_sid: self.sms_sid.expect("missing 'sms_sid' field"),
            from_state: self.from_state.expect("missing 'from_state' field"),
            sms_status: self.sms_status.expect("missing 'sms_status' field"),
            from_city: self.from_city.expect("missing 'from_city' field"),
            // TODO actually decode messages appropriately
            body: self.body.expect("missing 'body' field").replace("+", " "),
            from_country: self.from_country.expect("missing 'from_country' field"),
            to: self.to.expect("missing 'to' field"),
            to_zip: self.to_zip.is_some(),
            num_segments: self.num_segments.expect("missing 'num_segments' field"),
            message_sid: self.message_sid.expect("missing 'message_sid' field"),
            account_sid: self.account_sid.expect("missing 'account_sid' field"),
            from: self.from.expect("missing 'from' field"),
            api_version: self.api_version.expect("missing 'api_version' field"),
            media_urls: self.media_urls.unwrap_or_default(),
            media_content_types: self.media_content_types.unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct TwilioRequest {
    pub to_state: String,
    pub to_country: String,
    pub sms_message_sid: String,
    pub num_media: usize,
    pub to_city: String,
    pub from_zip: String,
    pub sms_sid: String,
    pub from_state: String,
    pub sms_status: String,
    pub from_city: String,
    pub body: String,
    pub from_country: String,
    pub to: String,
    pub to_zip: bool,
    pub num_segments: usize,
    pub message_sid: String,
    pub account_sid: String,
    pub from: String,
    pub api_version: String,
    pub media_urls: Vec<String>,
    pub media_content_types: Vec<String>,
}

impl TwilioRequest {
    pub fn builder() -> TwilioRequestBuilder {
        TwilioRequestBuilder::new()
    }

    // TODO maybe you can just use https://docs.rs/serde_urlencoded/0.7.0/serde_urlencoded/
    pub fn from_www_x_form_urlencoded(data: &str) -> Self {
        let mut builder = Self::builder();

        fn decode(s: &str) -> String {
            urlencoding::decode(&s).unwrap().to_string()
        }

        data.split('&').for_each(|kv| {
            let mut kv = kv.split('=');
            let key = kv.next().expect("empty keys are impossible");
            // Empty values are possible though
            let value = kv.next().map(decode);

            match key {
                "AccountSid" => {
                    builder.account_sid = value;
                }
                "ApiVersion" => {
                    builder.api_version = value;
                }
                "Body" => {
                    builder.body =  value;
                }
                "From" => {
                    builder.from = value;
                }
                "FromCity" => {
                    builder.from_city = value;
                }
                "FromCountry" => {
                    builder.from_country = value;
                }
                "FromState" => {
                    builder.from_state = value;
                }
                "FromZip" => {
                    builder.from_zip = value;
                }
                "MessageSid" => {
                    builder.message_sid = value;
                }
                "SmsMessageSid" => {
                    builder.sms_message_sid = value;
                }
                "SmsSid" => {
                    builder.sms_sid = value;
                }
                "SmsStatus" => {
                    builder.sms_status = value;
                }
                "To" => {
                    builder.to = value;
                }
                "ToCity" => {
                    builder.to_city = value;
                }
                "ToCountry" => {
                    builder.to_country = value;
                }
                "ToState" => {
                    builder.to_state = value;
                }
                "ToZip" => {
                    builder.to_zip = Some(());
                }
                "NumMedia" => {
                    builder.num_media = value.and_then(|v| v.parse().ok());
                }
                "NumSegments" => {
                    builder.num_segments = value.and_then(|v| v.parse().ok());
                }
                key if key.starts_with("MediaUrl") => {
                    // `get_or_insert_default` would be nicer but it's unstable
                    let urls = builder.media_urls.get_or_insert_with(Default::default);
                    urls.push(value.expect("there was a media url field but it had no value"));
                }
                key if key.starts_with("MediaContentType") => {
                    // `get_or_insert_default` would be nicer but it's unstable
                    let content_types = builder
                        .media_content_types
                        .get_or_insert_with(Default::default);
                    content_types.push(
                        value.expect("there was a media_content_type field but it had no value"),
                    );
                }
                other => {
                    println!(
                        "encountered unknown field when parsing request from Twilio: {}",
                        other
                    );
                }
            }
        });

        builder.build()
    }
}
