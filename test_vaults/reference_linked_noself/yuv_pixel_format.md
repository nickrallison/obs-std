---
bad_links:
aliases:
  - yuv
tags:
  - computergraphics
  - "imageprocessing"
title: YUV Pixel Format
---

# YUV Pixel Format

The YUV pixel format is a color encoding system used in various applications related to computer graphics and image processing. It plays a significant role in video compression and broadcasting, where it offers advantages in terms of better compression and reduction of [[Signal Bandwidth.md|bandwidth]] without significantly compromising the visual quality of images and videos. The format separates the image [[Information Theory|information]] into one [[Luminance|luminance]] component (Y) and two chrominance components (U and V).

## Understanding YUV Components

### [[Luminance|Luminance]] (Y)
The Y component represents the brightness level ([[luminance.md|luminance]]) of a pixel. This is the grayscale representation of the image, where higher values indicate brighter pixels. The human eye is more sensitive to variations in brightness than to variations in color, which is why [[Luminance|luminance]] is kept as a separate component in the YUV format.

### Chrominance (U and V)
The U and V components represent the color [[Information Theory|information]] of the image, also known as chrominance. The U component measures the blue projection of the color minus the [[Luminance|luminance,]] while the V component measures the red projection minus the [[Luminance|luminance.]] These components allow the representation of colors in the image. By combining the U and V values with the Y value, it is possible to reconstruct the original color information.

## Why YUV Format is Used

1. **Human Vision Sensitivity**: Since humans are more sensitive to changes in brightness than in color, separating these components allows systems to dedicate more resources to accurately representing brightness and less to color without noticeably affecting perceived image quality.

2. **Compression**: YUV format lends itself well to compression. By subsampling the U and V components (reducing the resolution of chrominance information), considerable data savings can be achieved. Common subsampling schemes include 4:4:4 (no subsampling), 4:2:2 (reduced horizontal color resolution), and 4:2:0 (reduced both horizontal and vertical color resolution).

3. **Compatibility and Standards**: Many video standards, including those used in DVD, Blu-ray, and television broadcast, are based on YUV or similar color spaces. Working directly in YUV format can simplify processes in video editing and playback systems.

## Applications of YUV Format

- **Video Compression**: Most video codecs (H.264, [[h264_vs_h265.md|HEVC]]) operate on YUV data to efficiently compress video content.
- **Image Processing**: Tasks such as noise reduction, color balancing, and contrast adjustment can be more intuitively performed in the YUV space.
- **Broadcasting**: Television signals are often broadcasted in YUV format to optimize [[Signal Bandwidth.md|bandwidth]] use while maintaining picture quality.

## Conclusion

The YUV pixel format's separation of [[Luminance|luminance]] and chrominance [[Information Theory|information]] is instrumental in optimizing video and image processing tasks for human perception, compression efficiency, and compatibility with existing standards and devices. Its application ranges from professional video editing to consumer media playback, underlining its importance in both computer graphics and image processing fields.