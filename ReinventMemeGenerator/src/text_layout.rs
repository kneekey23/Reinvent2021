use std::collections::VecDeque;
use image::{DynamicImage, GenericImageView};
use rusttype::{point, Font, PositionedGlyph, Rect, Scale};
use log::debug;

// A naive method of figuring out max line width in characters for a given font
pub fn max_chars_before_line_break(img: &DynamicImage, font: &Font, scale: Scale) -> usize {
    let mut m = "M".to_owned();

    loop {
        let m_plus_one = m.clone() + "M";
        if text_size(scale, &font, &m_plus_one).0 as u32 > img.width() {
            break m.len();
        }
        m = m_plus_one;
    }
}

pub fn break_text_up_dynamically(txt: &str, img: &DynamicImage, font: &Font, scale: Scale) -> Vec<String> {
    if txt.is_empty() {
        return Vec::new();
    }

    let mut words: VecDeque<&str> = txt.split(' ').collect();
    debug!("breaking up text with wordcount {}", words.len());
    let mut lines = Vec::new();

    while !words.is_empty() {
        let mut new_line = String::new();

        while let Some(next_word) = words.pop_front() {
                let mut next_line_plus_new_word = new_line.clone();

                if !next_line_plus_new_word.is_empty() {
                    next_line_plus_new_word.push(' ');
                }

                next_line_plus_new_word.push_str(next_word);

                if text_size(scale, &font, &next_line_plus_new_word).0 as u32 > img.width() {
                    // Line is too long, put the word back and start a new line
                    words.push_front(next_word);
                    break;
                } else {
                    // Line isn't too long, let's have another go around
                    new_line = next_line_plus_new_word;
                }
        };

        lines.push(new_line);
    }

    lines
}

// Doesn't respect anything. Produces bad line breaks but it's very simple at least
pub fn break_into_lines(text: &str, max_chars_before_line_break: usize) -> Vec<String> {
    text.chars()
        .collect::<Vec<char>>()
        .chunks(max_chars_before_line_break)
        .map(|c| c.iter().collect())
        .collect()
}

// TODO I copied this out of https://github.com/image-rs/imageproc/pull/453/files
// Once it's released I'll use that instead

fn layout_glyphs(
    scale: Scale,
    font: &Font,
    text: &str,
    mut f: impl FnMut(PositionedGlyph, Rect<i32>),
) -> (i32, i32) {
    use std::cmp::max;
    let v_metrics = font.v_metrics(scale);

    let (mut w, mut h) = (0, 0);

    for g in font.layout(text, scale, point(0.0, v_metrics.ascent)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = max(w, bb.max.x);
            h = max(h, bb.max.y);
            f(g, bb);
        }
    }

    (w, h)
}

/// Get the width and height of the given text, rendered with the given font and scale. Note that this function *does not* support newlines, you must do this manually.
pub fn text_size(scale: Scale, font: &Font, text: &str) -> (i32, i32) {
    layout_glyphs(scale, font, text, |_, _| {})
}
