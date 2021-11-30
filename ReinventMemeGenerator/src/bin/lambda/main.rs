// This example requires the following input to succeed:
// { "command": "do something" }

// mod message;
// mod twilio_request;
// mod twiml;

// use crate::message::Message;
// use crate::twilio_request::TwilioRequest;
// use crate::twiml::Twiml;
// use anyhow::Context;
// use image::EncodableLayout;
use lambda_runtime::handler_fn;
// use log::LevelFilter;
// use log::{debug, error, info, warn};
// use once_cell::unsync::Lazy;
// use reinvent_2021_rust::memeify::{extract_meme_text, memeify_image};
// use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
// use simple_logger::SimpleLogger;
//
// const BUCKET_NAME: Lazy<String> = Lazy::new(|| match std::env::var("BUCKET_NAME") {
//     Ok(var) => var,
//     Err(err) => {
//         warn!("BUCKET_NAME var error, returning empty string: {}", err);
//         Default::default()
//     }
// });
//
// #[tokio::main]
// async fn main() -> Result<(), lambda_runtime::Error> {
//     // required to enable CloudWatch error logging by the runtime
//     // can be replaced with any other method of initializing `log`
//     SimpleLogger::new()
//         .with_level(LevelFilter::Debug)
//         .init()
//         .unwrap();
//
//     let func = handler_fn(my_handler);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }
//
// async fn my_handler(
//     req: serde_json::Value,
//     _ctx: lambda_runtime::Context,
// ) -> Result<String, Error> {
//     let req: Request = serde_json::from_str(&req.to_string())
//         .with_context(|| format!("failed to deserialize this string as a Request: {}", req))?;
//
//     if req.body.is_empty() {
//         panic!(
//             "{}",
//             r#"Request body was empty, please supply a JSON object in this format: { "body": "the request body" }"#
//         )
//     }
//
//     let twilio_req = TwilioRequest::from_www_x_form_urlencoded(&req.body);
//     debug!(
//         "received a {} image request from Twilio: {}",
//         twilio_req.media_urls.len(),
//         twilio_req.body
//     );
//
//     let mut output = Twiml::new();
//     if twilio_req.media_urls.is_empty() {
//         output.add(&Message {
//             txt: format!("Hi '{}'. It looks like you didn't send a picture. Please send a picture next time.", twilio_req.from),
//         });
//     } else if twilio_req.media_urls.len() > 1 {
//         output.add(&Message {
//
//             txt: format!("Hi '{}'. It looks like you sent more than one picture. Please send only one picture next time.", twilio_req.from),
//         });
//     } else {
//         output.add(&Message {
//             txt: format!("Hi '{}'. Thanks for your picture", twilio_req.from),
//         });
//
//         let key = process_image(&twilio_req).await?;
//
//         info!(
//             "stored new image in bucket {} with key {}",
//             &*BUCKET_NAME, &key
//         );
//
//         // Notify the Kotlin backend that a new meme has arrived
//         let s3_uri = format!("s3://{}/{}", &*BUCKET_NAME, &key);
//         let meme_submission_url = "http://reinvent2021webserv-env.eba-zn2fjn6v.us-east-1.elasticbeanstalk.com/memes/submit";
//         let res = reqwest::Client::new()
//             .post(meme_submission_url)
//             .json(&serde_json::json!({
//                 "s3Uri": &s3_uri,
//             }))
//             .send()
//             .await?;
//
//         match res.status() {
//             StatusCode::OK => {
//                 if let Ok(body) = res.text().await {
//                     debug!("Kotlin server replied OK with body: {:?}", body);
//                     debug!("successfully submitted new meme to Kotlin server");
//                 } else {
//                     warn!("got 200 response from Kotlin but couldn't decode the body")
//                 }
//             }
//             _ => {
//                 if let Ok(body) = res.text().await {
//                     error!("Kotlin server replied non-OK status with body: {:?}", body);
//                 } else {
//                     warn!("got non-200 response from Kotlin but couldn't decode the body")
//                 }
//             }
//         }
//     }
//
//     // Superstitious trim
//     let output = output.as_twiml().trim().to_owned();
//
//     debug!("sending this Twiml back to Twilio: {}", &output);
//
//     // TODO Something is adding an extra newline which makes it so Twilio can't read the output. I blame API Gateway
//     Ok(output)
// }
//
// // returns the key of the image
// async fn process_image(twilio_req: &TwilioRequest) -> Result<String, Error> {
//     let config = aws_config::load_from_env().await;
//     let s3_client = aws_sdk_s3::Client::new(&config);
//
//     debug!("downloading image from Twilio...");
//     let picture = reqwest::get(&twilio_req.media_urls[0])
//         .await
//         .with_context(|| {
//             format!(
//                 "failed to download image from Twilio URL {}",
//                 &twilio_req.media_urls[0],
//             )
//         })?;
//     let bytes = picture
//         .bytes()
//         .await
//         .context("failed to collect bytes from twilio req")?;
//     debug!("image download complete");
//
//     let content_type = twilio_req.media_content_types[0].as_str();
//     let (top_text, bottom_text) = extract_meme_text(&twilio_req.body);
//
//     debug!("overlaying text onto image...");
//     let memed_image = memeify_image(bytes.as_bytes(), content_type, top_text, bottom_text)?;
//     debug!("overlaying complete");
//
//     let extension = get_file_extension(&content_type);
//
//     let key = if extension.is_empty() {
//         uuid::Uuid::new_v4().to_string()
//     } else {
//         format!("{}.{}", uuid::Uuid::new_v4().to_string(), extension)
//     };
//
//     debug!("uploading meme to S3...");
//     // Upload meme to S3
//     let _res = s3_client
//         .put_object()
//         .bucket(&*BUCKET_NAME)
//         .body(memed_image.into())
//         .key(&key)
//         .content_type("image/png")
//         .send()
//         .await
//         .with_context(|| {
//             format!(
//                 "failed to upload image to S3 bucket {} with key {}",
//                 &*BUCKET_NAME, &key
//             )
//         })?;
//     debug!("successfully uploaded meme to S3");
//
//     Ok(key)
// }
//
// fn get_file_extension(mime_type: &str) -> &str {
//     match mime_type {
//         "image/bmp" => "bmp",
//         "image/gif" => "gif",
//         "image/jpeg" => "jpg",
//         "image/png" => "png",
//         _ => {
//             warn!(
//                 "unhandled content type '{}', setting no extension",
//                 mime_type
//             );
//             ""
//         }
//     }
// }
