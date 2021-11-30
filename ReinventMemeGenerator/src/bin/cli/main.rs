/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

mod error;

use crate::error::AppError;
use anyhow::Context;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};
use log::{info, error, LevelFilter};
use reinvent_2021_rust::memeify::memeify_image;
use simple_logger::SimpleLogger;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const ARG_TOP_TEXT: &str = "top_text";
const ARG_BOTTOM_TEXT: &str = "bottom_text";

fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("meme")
                .about("Generate a meme from an image and some text")
                .arg_from_usage("<image> -i, --image=<PATH> 'the file path of the image to be memed'")
                .arg_from_usage("<output> -o, --output=<PATH> 'the file path where the memed image will be saved'")
                .arg(Arg::with_name(ARG_TOP_TEXT).long(ARG_TOP_TEXT).short("t").takes_value(true).value_name("TOP_TEXT").required_unless(ARG_BOTTOM_TEXT).help("the top meme text"))
                .arg(Arg::with_name(ARG_BOTTOM_TEXT).long(ARG_BOTTOM_TEXT).short("b").takes_value(true).value_name("BOTTOM_TEXT").required_unless(ARG_TOP_TEXT).help("the bottom meme text")),
        );

    if let Some(matches) = app.get_matches().subcommand_matches("meme") {
        meme_command(matches)?;
    }

    Ok(())
}

fn meme_command(matches: &ArgMatches) -> anyhow::Result<()> {
    let img_path = matches
        .value_of_os("image")
        .ok_or(AppError::ImagePathIsRequired)
        .map(|path| {
            info!("image_path: {:?}", path);
            PathBuf::from(path)
        })?;
    let output_path = matches
        .value_of_os("output")
        .ok_or(AppError::OutputPathIsRequired)
        .map(|path| {
            info!("output_path: {:?}", path);
            PathBuf::from(path)
        })?;
    let top_text = matches
        .value_of(ARG_TOP_TEXT);
    let bottom_text = matches
        .value_of(ARG_BOTTOM_TEXT);

    let img = fs::read(&img_path).context("image file not found")?;
    info!("successfully loaded image from {:?}", &img_path);

    let memed_img = match memeify_image(&img, "image/png", top_text, bottom_text) {
        Ok(img) => img,
        Err(err) => {
            return Err(anyhow::anyhow!("failed to meme image: {}", err));
        }
    };

    let output_file = File::create(&output_path).unwrap();
    let mut writer = BufWriter::new(output_file);
    if let Err(err) = writer.write(&memed_img.to_vec()) {
        error!("failed to write image to output file: {}", err);
    };

    info!(
        "Congratulations, your new meme is located at {:?}",
        &output_path
    );

    Ok(())
}
