use crate::print::{PrintError, PrintSettings, Printer, PrinterInfo, Result};
use std::path::Path;

pub struct StubPrinter;

impl Printer for StubPrinter {
    fn get_printers() -> Result<Vec<PrinterInfo>> {
        Err(PrintError::PlatformError(
            "Printing not supported on this platform".to_string(),
        ))
    }

    fn print_pdf(
        _pdf_path: &Path,
        _settings: &PrintSettings,
        _printer_name: Option<&str>,
    ) -> Result<()> {
        Err(PrintError::PlatformError(
            "Printing not supported on this platform".to_string(),
        ))
    }

    fn show_print_dialog(_pdf_path: &Path) -> Result<()> {
        Err(PrintError::PlatformError(
            "Printing not supported on this platform".to_string(),
        ))
    }
}

pub use StubPrinter as PlatformPrinter;
