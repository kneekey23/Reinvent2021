use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum AppError {
    #[error("An image path is required, please pass the path to an image with the --image arg")]
    ImagePathIsRequired,
    #[error(
        "An output path is required, please pass a path for saving output with the --output arg"
    )]
    OutputPathIsRequired,
}
