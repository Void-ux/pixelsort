# PixelSort

PixelSort is a customisable CLI program that manipulates images through sorting columns or rows of pixels, producing unique "glitchy" yet beautiful effects. 

## Showcase
![showcase image](https://cdn.overseer-bot.net/file/the-void/screenshots/output.png)

*This image was produced with the default pixelsort arguments*

## Features

Before delving into the features, it is worth understanding how pixelsort works. Essentially, pixels are taken either by columns (vertically) or by rows (horizontally) and sorted. Certain pixels can be excluded from the sort (so their position is never moved) through certain critera, for example, if a pixel is bright enough, it won't be moved.

- Pixelsort is **very fast**, programmed in Rust with attention to programmatic design choice.
- Pixelsort is generally very customisable, with different pixel exclusion algorithms allowing user-definable thresholds.
- Pixelsort supports many different formats, such as **png**, **webp** and **jpeg**.

## Installation

Currently only manual installation is supported while pixelsort is pre-release.

### Manual

#### Downloading

You can download pre-built binaries over on the [releases page](https://github.com/Void-ux/pixelsort/releases).

#### Building

Feel free to build and install from the source code directly (this requires the latest Rust compiler).
```shell
cargo install --git https://github.com/Void-ux/pixelsort.git pixelsort
```

### Usage

Pixelsort is relatively simple to use, simply run:
```shell
$ pixelsort image.png
```

This will apply the default arguments. You can customize pixelsort using arguments, for example:

```shell
$ pixelsort image.png -r 90 --exclude random_exclude --sort saturation -o sorted_image.png
```

Which will:

1. Sort the pixels horizontally instead of vertically.
2. Exclude pixels randomly, meaning that each pixel will have a 20% chance of not "moving".
3. Sort the pixels based by their saturation in ascending order. So the "blandest" pixels will be at the top of the column.
4. Output the sorted image as "sorted_image.png" in the current directory.

For more information on arguments/options, run
```shell
$ pixelsort --help
```
