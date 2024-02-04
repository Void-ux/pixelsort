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
