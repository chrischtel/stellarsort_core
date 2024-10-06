use image::DynamicImage;
use imageproc::filter::{laplacian_filter, gaussian_blur_f32};
use thiserror::Error;
use log::{info, warn, error};
use rayon::prelude::*;

/// Benutzerdefinierter Fehler für die Bildanalyse
#[derive(Error, Debug)]
pub enum ImageAnalysisError {
    #[error("Fehler beim Laden des Bildes: {0}")]
    ImageLoadError(String),
    
    #[error("Unbekannter Fehler")]
    Unknown,
}


pub fn calculate_variance(image: &imageproc::definitions::Image<image::Luma<i16>>) -> f64 {
    let mut sum: f64 = 0.0;
    let mut sum_squared: f64 = 0.0;
    let pixel_count = (image.width() * image.height()) as f64;

    for pixel in image.pixels() {
        let value = pixel[0] as f64;
        sum += value;
        sum_squared += value * value;
    }

    let mean = sum / pixel_count;
    (sum_squared / pixel_count) - (mean * mean)
}

/// Erkennt, ob ein Bild unscharf ist basierend auf der Varianz der Laplacian-Ergebnisse.
pub fn detect_blur(image: DynamicImage, blur_threshold: f64, denoise_sigma: f32) -> bool {
    info!("Starting blur detection");

    // Schritt 1: In Graustufen konvertieren
    let grayscale_image = image.to_luma8();

    let denoised_image = gaussian_blur_f32(&grayscale_image, denoise_sigma);

    // Schritt 2: Laplacian-Filter anwenden
    let laplacian = laplacian_filter(&denoised_image);

    // Schritt 3: Varianz der Laplacian-Ergebnisse berechnen
    let variance = calculate_variance(&laplacian);

    info!("Varianz des Bildes nach Rauschreduktion: {}", variance);
    // Schritt 4: Vergleichen mit dem Schärfe-Schwellenwert


    // Schritt 5: Vergleichen mit dem Schärfe-Schwellenwert
    if variance < blur_threshold {
        warn!("Bild ist unscharf (Varianz: {})", variance);
        true
    } else {
        info!("Bild ist scharf (Varianz: {})", variance);
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Luma};

    #[test]
    fn test_calculate_variance() {
        // Erstelle ein einfaches 2x2 Graustufenbild
        let img = ImageBuffer::from_fn(2, 2, |x, y| {
            if (x, y) == (0, 0) || (x, y) == (1, 1) {
                Luma([10])
            } else {
                Luma([20])
            }
        });

        let variance = calculate_variance(&img);
        assert_eq!(variance, 25.0);
    }

    #[test]
    fn test_detect_blur() {
        // Ein Bild mit wenig Kanten (unscharf)
        let blurry_img = ImageBuffer::from_fn(2, 2, |_, _| Luma([10]));
        let dynamic_blurry = DynamicImage::ImageLuma8(blurry_img);

        assert!(detect_blur(dynamic_blurry, 5.0, 1.0));

        // Ein Bild mit mehr Kanten (scharf)
        let sharp_img = ImageBuffer::from_fn(2, 2, |x, y| {
            if x == y {
                Luma([10])
            } else {
                Luma([30])
            }
        });
        let dynamic_sharp = DynamicImage::ImageLuma8(sharp_img);

        assert!(!detect_blur(dynamic_sharp, 5.0, 1.0));
    }
}