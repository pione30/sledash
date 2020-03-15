## Requirements

- Clang
  - See the [Requirements page](https://rust-lang.github.io/rust-bindgen/requirements.html) of [rust-bindgen](https://github.com/rust-lang/rust-bindgen).

- ImageMagick

- MagickWand headers files

- pkg-config

If the command below:

```
$ pkg-config --exists MagickWand; echo $?
```

does not show `0`, create MagickWand.pc under /usr/local/lib/pkgconfig according to your `convert` like this:

```
prefix=/usr
includedir=${prefix}/include
includearchdir=${prefix}/include/x86_64-linux-gnu

Name: MagickWand
Description: MagickWand - C API for ImageMagick
URL: https://github.com/ImageMagick
Version: 6.9.7
Cflags: -I${includedir}/ImageMagick-6 -I${includearchdir}/ImageMagick-6 -Xpreprocessor -fopenmp -DMAGICKCORE_HDRI_ENABLE=0 -DMAGICKCORE_QUANTUM_DEPTH=16
```
