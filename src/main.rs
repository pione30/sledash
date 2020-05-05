use std::convert::TryInto;
use std::env;
use std::fs::DirBuilder;
use std::io::Write;
use std::path::Path;
use tokio::task;

use indicatif::{ProgressBar, ProgressStyle};
use regex::RegexSet;

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

    let response = emoji_list::fetch(&token).await.unwrap_or_else(|error| {
        panic!("emoji_list::fetch failed: {}", error);
    });
    if !response.ok {
        eprintln!(
            "Error is returned from emoji.list API: {}",
            response
                .error
                .expect("Some error value should be present when the response_body.ok is false")
        );
        return;
    }

    let emoji = response
        .emoji
        .expect("emoji hash should exist when response.ok is true");

    // ignore jpeg
    let ignored_extensions = RegexSet::new(&[r"jpe?g"]).unwrap();

    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.green} ([{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg})")
        .progress_chars("#>-");

    let emoji_progress_bar = ProgressBar::new(
        emoji
            .len()
            .try_into()
            .expect("Converting usize (emoji.len()) into u64 to be succeeded"),
    );

    emoji_progress_bar.set_style(progress_style.clone());
    emoji_progress_bar.set_message("emoji");

    // HTTP request client
    let client = reqwest::Client::new();

    task::spawn(async move {
        for (emoji_name, emoji_url) in &emoji {
            // flushing to the terminal is a heavy task
            task::block_in_place(|| {
                emoji_progress_bar.inc(1);
            });

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

            // # TODO: dealing with gif animations
            if ignored_extensions.is_match(extension) || extension.contains("gif") {
                continue;
            }

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

            // saving emoji to the file is a heavy task
            if let Err(message) = task::block_in_place(|| {
                let file = std::fs::File::create(&emoji_save_path);
                if let Err(error) = file {
                    return Err(format!(
                        "Failed to create emoji file {}: {}",
                        &emoji_save_path.display(),
                        error
                    ));
                }
                let mut file = file.unwrap();

                // save emoji bytes
                if let Err(error) = file.write_all(bytes.as_ref()) {
                    return Err(format!(
                        "Failed to write bytes to file {}: {}",
                        &emoji_save_path.display(),
                        error
                    ));
                }

                Ok(())
            }) {
                eprintln!("{}", message);
                continue;
            }

            // wand to be taken by all the MagickWandy APIs
            let mut wand = magickwand::Wand::new();

            if let Err(exception_type) = task::block_in_place(|| {
                let mut input_emoji =
                    magickwand::File::new(&emoji_save_path.to_string_lossy(), "rb");

                wand.magick_read_image_file(&mut input_emoji)
            }) {
                eprintln!(
                    "magick_read_image_file {} failed: {}",
                    &emoji_save_path.display(),
                    exception_type
                );
                continue;
            }

            wand.magick_reset_iterator();

            // Pixel set its color to white
            let mut pixel_white = magickwand::Pixel::new();
            if let Err(exception_type) = pixel_white.pixel_set_color("white") {
                eprintln!("Failed to set Pixel to white: {}", exception_type);
                continue;
            }

            while wand.magick_next_image().is_some() {
                // use for shadowing the clone of the original emoji
                let mut shadow_clone = wand.clone_magick_wand();

                if let Err(exception_type) = task::block_in_place(|| {
                    shadow_clone.magick_set_image_background_color(&pixel_white)
                }) {
                    eprintln!(
                        "magick_set_image_background_color to white {} failed: {}",
                        &emoji_save_path.display(),
                        exception_type
                    );
                    break;
                }

                if let Err(exception_type) =
                    task::block_in_place(|| shadow_clone.magick_shadow_image(100.0, 8.0, 0, 0))
                {
                    eprintln!(
                        "magick_shadow_image {} failed: {}",
                        &emoji_save_path.display(),
                        exception_type
                    );
                    break;
                }

                if let Err(exception_type) =
                    task::block_in_place(|| shadow_clone.magick_reset_image_page(""))
                {
                    eprintln!(
                        "magick_reset_image_page {} failed: {}",
                        &emoji_save_path.display(),
                        exception_type
                    );
                    break;
                }

                if let Err(exception_type) = task::block_in_place(|| {
                    wand.magick_composite_image_gravity(
                        &shadow_clone,
                        magickwand::CompositeOperator::DstOverCompositeOp,
                        magickwand::GravityType::CenterGravity,
                    )
                }) {
                    eprintln!(
                        "magick_composite_image_gravity {} failed: {}",
                        &emoji_save_path.display(),
                        exception_type
                    );
                    break;
                }
            }

            if let Err(exception_type) = task::block_in_place(|| {
                let mut output_emoji =
                    magickwand::File::new(&emoji_save_path.to_string_lossy(), "wb");
                // *images* to deal with gif animations
                wand.magick_write_images_file(&mut output_emoji)
            }) {
                eprintln!(
                    "magick_write_images_file {} failed: {}",
                    &emoji_save_path.display(),
                    exception_type
                );
            }
        }

        task::block_in_place(|| {
            emoji_progress_bar.finish_with_message("done");
        });
    })
    .await
    .expect("wand_task to complete");

    // magickwand::magick_wand_terminus();

    println!("emojis are saved under {} directory.", emoji_save_directory);
}
