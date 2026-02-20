pub mod platform;

use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrintError {
    #[error("Failed to print: {0}")]
    PrintError(String),
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    #[error("Platform error: {0}")]
    PlatformError(String),
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    #[error("Initialization error: {0}")]
    InitError(String),
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    #[error("No printer available")]
    NoPrinter,
}

pub type Result<T> = std::result::Result<T, PrintError>;

#[cfg(any(target_os = "windows", target_os = "linux"))]
#[derive(Debug, Clone)]
pub struct PrinterInfo {
    pub name: String,
    pub is_default: bool,
    pub supports_color: bool,
    pub supports_duplex: bool,
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
#[derive(Debug, Clone, Copy)]
pub enum PaperSize {
    A4,
    A3,
    A5,
    Letter,
    Legal,
    Tabloid,
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
impl PaperSize {
    pub fn dimensions_mm(&self) -> (f32, f32) {
        match self {
            PaperSize::A4 => (210.0, 297.0),
            PaperSize::A3 => (297.0, 420.0),
            PaperSize::A5 => (148.0, 210.0),
            PaperSize::Letter => (216.0, 279.0),
            PaperSize::Legal => (216.0, 356.0),
            PaperSize::Tabloid => (279.0, 432.0),
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
#[derive(Debug, Clone)]
pub struct PageRange {
    pub start: usize,
    pub end: usize,
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
impl PageRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn all(page_count: usize) -> Self {
        Self {
            start: 0,
            end: page_count.saturating_sub(1),
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
#[derive(Debug, Clone, Copy)]
pub struct Margins {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 10.0,
            right: 10.0,
            bottom: 10.0,
            left: 10.0,
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
#[derive(Debug, Clone)]
pub struct PrintSettings {
    pub paper_size: PaperSize,
    pub orientation: Orientation,
    pub page_range: Option<PageRange>,
    pub copies: u32,
    pub duplex: bool,
    pub color: bool,
    pub scale_to_fit: bool,
    pub margins: Margins,
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
impl Default for PrintSettings {
    fn default() -> Self {
        Self {
            paper_size: PaperSize::A4,
            orientation: Orientation::Portrait,
            page_range: None,
            copies: 1,
            duplex: false,
            color: true,
            scale_to_fit: true,
            margins: Margins::default(),
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
pub trait Printer {
    fn get_printers() -> Result<Vec<PrinterInfo>>;
    fn print_pdf(
        pdf_path: &Path,
        settings: &PrintSettings,
        printer_name: Option<&str>,
    ) -> Result<()>;
    fn show_print_dialog(pdf_path: &Path) -> Result<()>;
}

pub use platform::PlatformPrinter;

pub fn show_print_dialog(pdf_path: &Path) -> Result<()> {
    PlatformPrinter::show_print_dialog(pdf_path)
}
