use base64::Engine;
use pdfium_auto::PdfiumAutoError;
use pdfium_render::prelude::*;

fn bind_pdfium() -> Result<Pdfium, String> {
    pdfium_auto::bind_pdfium_silent()
        .map_err(|e: PdfiumAutoError| format!("Failed to load PDFium: {}. Try setting PDFIUM_LIB_PATH environment variable to a local libpdfium binary.", e))
}

/// Render each page of a PDF to PNG images, returned as base64 data URLs.
pub fn render_pdf_pages(
    pdf_bytes: &[u8],
    zoom: f64,
    device_pixel_ratio: f64,
) -> Result<Vec<String>, String> {
    let pdfium = bind_pdfium()?;

    let document = pdfium
        .load_pdf_from_byte_slice(pdf_bytes, None)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let scale = zoom * device_pixel_ratio;
    let mut pages = Vec::new();

    for (index, page) in document.pages().iter().enumerate() {
        let width_px = ((page.width().value * scale as f32).ceil() as i32).max(1);
        let height_px = ((page.height().value * scale as f32).ceil() as i32).max(1);

        let render_config = PdfRenderConfig::new()
            .set_target_width(width_px)
            .set_target_height(height_px);

        let bitmap = page
            .render_with_config(&render_config)
            .map_err(|e| format!("Failed to render page {}: {}", index + 1, e))?;

        let dynamic_image = bitmap.as_image();

        let mut png_bytes: Vec<u8> = Vec::new();
        dynamic_image
            .write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .map_err(|e| format!("Failed to encode PNG for page {}: {}", index + 1, e))?;

        let base64_string = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
        pages.push(format!("data:image/png;base64,{}", base64_string));
    }

    Ok(pages)
}
