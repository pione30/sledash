use std::env;
use std::fs::DirBuilder;
use std::io::Write;
use std::path::Path;

use magickwand;
use reqwest;

mod emoji_list;

#[tokio::main]
async fn main() {
    let env_key = "SLACK_APP_ACCESS_TOKEN";
    let token = env::var(env_key);
    if let Err(error) = token {
        eprintln!(
            "Failed to fetch SLACK_APP_ACCESS_TOKEN environment variable: {}",
            error
        );
        return;
    }
    let token = token.unwrap();

    // create directory to save emoji
    let emoji_save_directory = "slack_emoji";
    DirBuilder::new()
        .recursive(true)
        .create(emoji_save_directory)
        .expect("the directory should be created when recursive mode is enabled");

    let response = emoji_list::fetch(&token).await;
    if !response.ok {
        eprintln!(
            "Error is returned from emoji.list API: {}",
            response
                .error
                .expect("Some error value should be present when the response_body.ok is false")
        );
        return;
    }

    // Pixel set its color to white
    let mut pixel_white = magickwand::Pixel::new();
    if let Err(exception_type) = pixel_white.pixel_set_color("white") {
        eprintln!("Failed to set Pixel to white: {}", exception_type);
        return;
    }

    // HTTP request client
    let client = reqwest::Client::new();

    'emoji: for (emoji_name, emoji_url) in &response
        .emoji
        .expect("emoji hash should exist when response.ok is true")
    {
        // skip aliases
        if emoji_url.starts_with("alias") {
            continue;
        }

        let extension = if let Some(index) = emoji_url.rfind('.') {
            &emoji_url[index..emoji_url.len()]
        } else {
            eprintln!("{} is not a valid emoji url", emoji_url);
            continue;
        };

        let response = client.get(emoji_url).send().await;
        if let Err(error) = response {
            eprintln!("HTTP GET {} failed: {}", emoji_url, error);
            continue;
        }
        let response = response.unwrap();

        let bytes = response.bytes().await;
        if let Err(error) = bytes {
            eprintln!(
                "Failed to get the full emoji {} response body as bytes: {}",
                emoji_name, error
            );
            continue;
        }
        let bytes = bytes.unwrap();

        let emoji_filename = format!("{}{}", emoji_name, extension);
        let emoji_save_path = Path::new(emoji_save_directory).join(emoji_filename);
        let file = std::fs::File::create(&emoji_save_path);
        if let Err(error) = file {
            eprintln!(
                "Failed to create emoji file {}: {}",
                &emoji_save_path.display(),
                error
            );
            continue;
        }
        let mut file = file.unwrap();

        // save emoji bytes
        if let Err(error) = file.write_all(bytes.as_ref()) {
            eprintln!(
                "Failed to write bytes to file {}: {}",
                &emoji_save_path.display(),
                error
            );
            continue;
        }

        // wand to be taken by all the MagickWandy APIs
        let mut wand = magickwand::Wand::new();

        {
            let mut input_emoji = magickwand::File::new(&emoji_save_path.to_string_lossy(), "rb");

            if let Err(exception_type) = wand.magick_read_image_file(&mut input_emoji) {
                eprintln!(
                    "magick_read_image_file {} failed: {}",
                    &emoji_save_path.display(),
                    exception_type
                );
                continue;
            }
        }

        wand.magick_reset_iterator();
        while wand.magick_next_image().is_some() {
            // use for shadowing the clone of the original emoji
            let mut shadow_clone = wand.clone_magick_wand();

            if let Err(exception_type) =
                shadow_clone.magick_set_image_background_color(&pixel_white)
            {
                eprintln!(
                    "magick_set_image_background_color to white {} failed: {}",
                    &emoji_save_path.display(),
                    exception_type
                );
                continue 'emoji;
            }

            if let Err(exception_type) = shadow_clone.magick_shadow_image(100.0, 8.0, 0, 0) {
                eprintln!(
                    "magick_shadow_image {} failed: {}",
                    &emoji_save_path.display(),
                    exception_type
                );
                continue 'emoji;
            }

            if let Err(exception_type) =
                wand.magick_set_image_gravity(magickwand::GravityType::CenterGravity)
            {
                eprintln!(
                    "magick_set_image_gravity {} failed: {}",
                    &emoji_save_path.display(),
                    exception_type
                );
                continue 'emoji;
            }

            if let Err(exception_type) = wand.magick_composite_image(
                &shadow_clone,
                magickwand::CompositeOperator::DstOverCompositeOp,
                0,
                0,
            ) {
                eprintln!(
                    "magick_composite_image {} failed: {}",
                    &emoji_save_path.display(),
                    exception_type
                );
                continue 'emoji;
            }
        }

        {
            let mut output_emoji = magickwand::File::new(&emoji_save_path.to_string_lossy(), "wb");
            // *images* to deal with gif animations
            if let Err(exception_type) = wand.magick_write_images_file(&mut output_emoji) {
                eprintln!(
                    "magick_write_images_file {} failed: {}",
                    &emoji_save_path.display(),
                    exception_type
                );
                continue;
            }
        }
    }
    // magickwand::magick_wand_terminus();

    println!("emojis are saved under {} directory.", emoji_save_directory);
}
