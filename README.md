[![crates.io](https://img.shields.io/crates/v/sledash.svg)](https://crates.io/crates/sledash)
[![docs.rs](https://docs.rs/sledash/badge.svg)](https://docs.rs/sledash)
![](https://github.com/pione30/sledash/workflows/Continuous%20integration/badge.svg)

## Sledash

![](assets/LGTM-dark-background.png)
![](assets/LGTM-shadow-dark-background.png)

Sledash is an abbreviation for *Slack Emoji Darkmode Shader*.

Fetch emoji lists from your Slack workspace and add white shades for better visibilities in the darkmode.

Set `SLACK_APP_ACCESS_TOKEN` environment variable to your Slack OAuth Access Token with `emoji:read` scope granted.

WIP: As of now, adding the shades to gif animations does not work well (some wierd animations will be generated.)

## Requirements

- Clang
  - See the [Requirements page](https://rust-lang.github.io/rust-bindgen/requirements.html) of [rust-bindgen](https://github.com/rust-lang/rust-bindgen).

- ImageMagick >= 7.0.10

- MagickWand headers files

- MagickWand development files

- pkg-config

If the command below:

```
$ pkg-config --exists MagickWand; echo $?
```

does not show `0`, something are missing. Check your environment again and install the missing packages.

## License

MIT OR Apache-2.0
