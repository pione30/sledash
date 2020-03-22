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
    let token =
        env::var(env_key).expect("SLACK_APP_ACCESS_TOKEN environment variable should be fetched");

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

    // wand to be taken by all the MagickWandy APIs
    let wand = magickwand::Wand::new();

    // HTTP request client
    let client = reqwest::Client::new();

    for (emoji_name, emoji_url) in &response
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

        {
            let input_emoji = magickwand::File::new(&emoji_save_path.to_string_lossy(), "rb");
            wand.magick_read_image_file(&input_emoji);
            wand.magick_reset_iterator();
        }

        {
            let output_emoji = magickwand::File::new(&emoji_save_path.to_string_lossy(), "wb");
            wand.magick_write_image_file(&output_emoji);
        }

        wand.clear_magick_wand();
    }
    // magickwand::magick_wand_terminus();
}
