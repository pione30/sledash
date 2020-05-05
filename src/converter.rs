use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConverterError {
    #[error("failed to read image from the file: {0}")]
    ReadImageFileFailed(sledash_magickwand::ExceptionType),
    #[error("failed to set Pixel color to {color}: {exception_type}")]
    PixelSetColorFailed {
        exception_type: sledash_magickwand::ExceptionType,
        color: String,
    },
    #[error("failed to set image background color: {0}")]
    SetImageBackgroundColorFailed(sledash_magickwand::ExceptionType),
    #[error("failed to shadow image: {0}")]
    ShadowImageFailed(sledash_magickwand::ExceptionType),
    #[error("failed to reset image page: {0}")]
    ResetImagePageFailed(sledash_magickwand::ExceptionType),
    #[error("failed to compose image gravity: {0}")]
    ComposeImageGravityFailed(sledash_magickwand::ExceptionType),
    #[error("failed to write images to the file: {0}")]
    WriteImagesFileFailed(sledash_magickwand::ExceptionType),
}

/// Adds a shadow to the image specified with `emoji_path`.
///
/// # Errors
///
/// This method fails if any of the magickwand methods returns an error.
pub fn add_shade(emoji_path: &std::path::Path) -> Result<(), ConverterError> {
    // wand to be taken by all the MagickWandy APIs
    let mut wand = sledash_magickwand::Wand::new();

    {
        let mut input_emoji = sledash_magickwand::File::new(&emoji_path.to_string_lossy(), "rb");

        wand.magick_read_image_file(&mut input_emoji)
            .map_err(ConverterError::ReadImageFileFailed)?;
    }

    wand.magick_reset_iterator();

    // Pixel set its color to white
    let mut pixel_white = sledash_magickwand::Pixel::new();
    pixel_white
        .pixel_set_color("white")
        .map_err(|exception_type| ConverterError::PixelSetColorFailed {
            exception_type,
            color: "white".to_string(),
        })?;

    while wand.magick_next_image().is_some() {
        // use for shadowing the clone of the original emoji
        let mut shadow_clone = wand.clone_magick_wand();

        shadow_clone
            .magick_set_image_background_color(&pixel_white)
            .map_err(ConverterError::SetImageBackgroundColorFailed)?;

        shadow_clone
            .magick_shadow_image(100.0, 8.0, 0, 0)
            .map_err(ConverterError::ShadowImageFailed)?;

        shadow_clone
            .magick_reset_image_page("")
            .map_err(ConverterError::ResetImagePageFailed)?;

        wand.magick_composite_image_gravity(
            &shadow_clone,
            sledash_magickwand::CompositeOperator::DstOverCompositeOp,
            sledash_magickwand::GravityType::CenterGravity,
        )
        .map_err(ConverterError::ComposeImageGravityFailed)?;
    }

    let mut output_emoji = sledash_magickwand::File::new(&emoji_path.to_string_lossy(), "wb");
    // *images* to deal with gif animations
    wand.magick_write_images_file(&mut output_emoji)
        .map_err(ConverterError::WriteImagesFileFailed)?;

    Ok(())
}
