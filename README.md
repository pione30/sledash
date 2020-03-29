## Slack Emoji Darkmode Shader

WIP: The only feature as of now is to fetch emoji lists from your Slack workspace.

Set `SLACK_APP_ACCESS_TOKEN` environment variable to your Slack OAuth Access Token with `emoji:read` scope granted.

## Requirements

- Clang
  - See the [Requirements page](https://rust-lang.github.io/rust-bindgen/requirements.html) of [rust-bindgen](https://github.com/rust-lang/rust-bindgen).

- ImageMagick 7.0

- MagickWand headers files

- MagickWand development files

- pkg-config

If the command below:

```
$ pkg-config --exists MagickWand; echo $?
```

does not show `0`, something are missing. Check your environment again and install the missing packages.
