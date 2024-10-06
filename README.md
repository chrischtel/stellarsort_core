# StellarSort Core Library

<p align="center">
  <img src="https://user-images.githubusercontent.com/yourusername/logo.png" alt="StellarSort Logo" width="200">
</p>

<p align="center">
  <b>A powerful Rust toolkit for analyzing and sorting astrophotography images.</b>
</p>

<p align="center">
  <a href="https://crates.io/crates/stellarsort_core">
    <img src="https://img.shields.io/crates/v/stellarsort_core.svg" alt="Crates.io Version">
  </a>
  <a href="https://docs.rs/stellarsort_core">
    <img src="https://docs.rs/stellarsort_core/badge.svg" alt="Documentation">
  </a>
  <a href="https://github.com/chrischtel/stellarsort/actions">
    <img src="https://github.com/yourusername/stellarsort_core/workflows/CI/badge.svg" alt="Build Status">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/github/license/chrischtel/stellarsort" alt="License">
  </a>


---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Example: Detecting Blurry Images](#example-detecting-blurry-images)
- [Roadmap](#roadmap)
- [API Reference](#api-reference)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

---

## Overview

**StellarSort Core Library** is a powerful Rust library specifically designed for analyzing and sorting astrophotography images. It provides tools for detecting blurry images, noise reduction, and other image quality analyses to assist astrophotographers in processing their images.

## Features

- **Detect Blurry Images**: Utilizes advanced algorithms to reliably identify blurry images.
- **Noise Reduction**: Integrated filters to reduce image noise without losing important details.
- **Modular Design**: Allows for easy extension and customization to meet individual requirements.
- **Easy to Use**: Well-documented API with examples for quick onboarding.

## Installation

Add the `stellarsort_core` library to your `Cargo.toml` dependencies:

```toml
[dependencies]
stellarsort_core = "0.1.0"
```

Make sure to check the latest version on [crates.io](https://crates.io/crates/stellarsort_core).

## Usage

Import the library into your Rust project and utilize the available image analysis functions.

### Example: Detecting Blurry Images

```rust
use stellarsort_core::detect_blur;
use image::DynamicImage;

fn main() {
    // Load an image
    let img = image::open("example_image.png").expect("Failed to load image");

    // Define the blur threshold and sigma value for noise reduction
    let blur_threshold = 100.0;
    let denoise_sigma = 1.0;

    // Check if the image is blurry
    if detect_blur(img, blur_threshold, denoise_sigma, None) {
        println!("The image is blurry.");
    } else {
        println!("The image is sharp.");
    }
}
```

## Roadmap

We are continuously working to improve StellarSort Core and add new features. Here is our current roadmap:

| Done                    | Active                | Planned                      |
|-------------------------|-----------------------|------------------------------|
| Detect Blurry Images    | Parallel Processing   | Machine Learning Integration |
| Noise Reduction         | Enhanced Testing      | Gradient Removal             |
| Modular Design          |                       | Custom Filters               |

*If you have ideas or suggestions, feel free to contact us or create an issue on GitHub!*

## API Reference

The full API documentation is available on [docs.rs](https://docs.rs/stellarsort_core). Here are some of the main functions:

- `detect_blur`: Detects blurry images based on the variance of Laplacian results.
- `calculate_variance`: Calculates the variance of pixel values in a grayscale image.
- `calculate_noise`: Measures the noise level of an image by calculating the standard deviation.
- `analyze_images_parallel`: Performs image analysis in parallel on a list of images.

## Contributing

Contributions are welcome! If you find bugs or want to suggest new features:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

Please ensure that your contributions are well-documented and tested.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

A big thank you to all the open-source projects and libraries that made this work possible:

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)
- [rayon](https://crates.io/crates/rayon)
- [thiserror](https://crates.io/crates/thiserror)
- [indicatif](https://crates.io/crates/indicatif)
- [log](https://crates.io/crates/log)
- [env_logger](https://crates.io/crates/env_logger)

---

<p align="center">
  Developed with ❤️ by <a href="https://github.com/chrischtel">yourusername</a>
</p>
