use crate::text_layout::{break_text_up_dynamically, text_size};
use anyhow::{anyhow, Context};
use bytes::Bytes;
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageFormat, Rgba};
use imageproc::drawing::draw_text_mut;
use log::{debug, error};
use rusttype::{Font, Scale};
use std::io::Cursor;

const TEXT_VERTICAL_PADDING: u32 = 20;
const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];
const WHITE: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

pub fn extract_meme_text(text: &str) -> (Option<&str>, Option<&str>) {
    if text.is_empty() {
        (Some("is this"), Some("a meme?"))
    } else {
        let mut iter = text.split('.').map(str::trim);

        let top_text = iter.next();
        let bottom_text = iter.next();

        (top_text, bottom_text)
    }
}

pub fn memeify_image(
    image_bytes: &[u8],
    image_format: &str,
    top_text: Option<&str>,
    bottom_text: Option<&str>,
) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
    let top_text = top_text.map(ToOwned::to_owned).unwrap_or_default();
    let bottom_text = bottom_text.map(ToOwned::to_owned).unwrap_or_default();

    debug!("reading image");
    let mut img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .context("failed to read bytes as image data")?
        .decode()
        .context("couldn't decode that image")?;

    debug!("loading font");
    let font = Vec::from(include_bytes!("impact.ttf") as &[u8]);
    let font = Font::try_from_vec(font).ok_or_else(|| anyhow!("failed to read font"))?;

    // Line height of text is 1/10th of image height
    let bg_height = img.height() as f32 / 10.0;
    let bg_scale = Scale {
        x: bg_height,
        y: bg_height,
    };
    let fg_height = bg_height * 0.98;
    let fg_scale = Scale {
        x: fg_height,
        y: fg_height,
    };

    // Draw top text
    {
        let (font_width, font_height) = text_size(bg_scale, &font, &top_text);
        let (font_width, font_height) = (font_width as u32, font_height as u32);
        debug!("top text starting size: {}x{}", font_width, font_height);
        let y = TEXT_VERTICAL_PADDING;

        if font_width <= img.width() {
            debug!("top text doesn't need wrapping");

            let x = (img.width() - font_width) / 2;
            draw_text_mut(&mut img, Rgba(BLACK), x, y, bg_scale, &font, &top_text);
            let x = (img.width() - text_size(fg_scale, &font, &top_text).0 as u32) / 2;
            draw_text_mut(&mut img, Rgba(WHITE), x, y, bg_scale, &font, &top_text);
        } else {
            debug!("top text needs wrapping, breaking it up...");

            let lines = break_text_up_dynamically(&top_text, &img, &font, bg_scale);
            for (i, line) in lines.iter().enumerate() {
                let (font_width, font_height) = text_size(bg_scale, &font, line);
                let x = (img.width() - font_width as u32) / 2;
                let y = (i as u32 * font_height as u32) + y;
                draw_text_mut(&mut img, Rgba(BLACK), x, y, bg_scale, &font, &line);
                let x = (img.width() - text_size(fg_scale, &font, &line).0 as u32) / 2;
                draw_text_mut(&mut img, Rgba(WHITE), x, y, fg_scale, &font, &line);
            }
        }
    }

    // Draw bottom text
    {
        let (font_width, font_height) = text_size(bg_scale, &font, &bottom_text);
        let (font_width, font_height) = (font_width as u32, font_height as u32);
        debug!("bottom text starting size: {}x{}", font_width, font_height);
        let y = img.height() - font_height - TEXT_VERTICAL_PADDING;

        if font_width <= img.width() {
            debug!("bottom text doesn't need wrapping");

            let x = (img.width() - font_width) / 2;
            draw_text_mut(&mut img, Rgba(BLACK), x, y, bg_scale, &font, &bottom_text);
            let x = (img.width() - text_size(fg_scale, &font, &bottom_text).0 as u32) / 2;
            draw_text_mut(&mut img, Rgba(WHITE), x, y, fg_scale, &font, &bottom_text);
        } else {
            debug!("bottom text needs wrapping, breaking it up...");

            let lines = break_text_up_dynamically(&bottom_text, &img, &font, bg_scale);
            for (i, line) in lines.iter().enumerate() {
                let (font_width, font_height) = text_size(bg_scale, &font, line);
                let x = (img.width() - font_width as u32) / 2;
                let y = ((i as u32) * font_height as u32) + y - (font_height as u32 * 2);
                draw_text_mut(&mut img, Rgba(BLACK), x, y, bg_scale, &font, &line);
                let x = (img.width() - text_size(fg_scale, &font, &line).0 as u32) / 2;
                draw_text_mut(&mut img, Rgba(WHITE), x, y, fg_scale, &font, &line);
            }
        }
    }

    let bytes = {
        let mut bytes: Vec<u8> = Vec::new();

        match image_format {
            "image/gif" => {
                debug!("re-encoding image as gif...");
                img.write_to(&mut bytes, ImageFormat::Gif).unwrap();
            }
            "image/jpeg" => {
                debug!("re-encoding image as jpeg...");
                img.write_to(&mut bytes, ImageFormat::Jpeg).unwrap();
            }
            "image/png" => {
                debug!("re-encoding image as png...");
                img.write_to(&mut bytes, ImageFormat::Png).unwrap();
            }
            "image/bmp" => {
                debug!("re-encoding image as bmp...");
                img.write_to(&mut bytes, ImageFormat::Bmp).unwrap();
            }
            mime_type => error!(
                "failed to encode image, unhandled MIME type '{}'",
                mime_type
            ),
        }

        debug!("re-encoding image complete");
        bytes
    };

    Ok(bytes.into())
}
