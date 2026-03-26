use xcap::Monitor;
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;
use anyhow::{Result, anyhow};

pub struct CaptureManager;

impl CaptureManager {
    pub async fn capture_and_compress() -> Result<Vec<u8>> {
        let buffer = tokio::task::spawn_blocking(move || {
            let monitors = Monitor::all().map_err(|e| anyhow!("Failed to list monitors: {:?}", e))?;
            
            let monitor = monitors.first()
                .ok_or_else(|| anyhow!("No monitors found for capture"))?;
                
            let image = monitor.capture_image()
                .map_err(|e| anyhow!("Failed to capture image: {:?}", e))?;
            
            let mut webp_buffer = Vec::new();
            let dynamic_image = DynamicImage::ImageRgba8(image);
            
            dynamic_image.write_to(&mut Cursor::new(&mut webp_buffer), ImageFormat::WebP)
                .map_err(|e| anyhow!("Failed to compress image to WebP: {:?}", e))?;
                
            Ok::<Vec<u8>, anyhow::Error>(webp_buffer)
        }).await.map_err(|e| anyhow!("Blocking task panicked: {:?}", e))??;

        Ok(buffer)
    }
}
