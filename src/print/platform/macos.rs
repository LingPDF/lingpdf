use crate::print::{PrintError, Result};
use std::path::Path;

#[cfg(target_os = "macos")]
use objc::runtime::Object;
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

pub struct MacOSPrinter;

impl MacOSPrinter {
    #[cfg(target_os = "macos")]
    pub fn show_print_dialog(pdf_path: &Path) -> Result<()> {
        use std::ffi::CString;

        let path_str = pdf_path
            .to_str()
            .ok_or_else(|| PrintError::PrintError("Invalid PDF path".to_string()))?;

        unsafe {
            let path_cstr = CString::new(path_str)
                .map_err(|_| PrintError::PrintError("Invalid path encoding".to_string()))?;

            let ns_string: *mut Object =
                msg_send![class!(NSString), stringWithUTF8String: path_cstr.as_ptr()];
            if ns_string.is_null() {
                return Err(PrintError::PrintError(
                    "Failed to create NSString".to_string(),
                ));
            }

            let ns_url: *mut Object = msg_send![class!(NSURL), fileURLWithPath: ns_string];
            if ns_url.is_null() {
                return Err(PrintError::PrintError("Failed to create NSURL".to_string()));
            }

            let pdf_doc: *mut Object = msg_send![class!(PDFDocument), alloc];
            let pdf_doc: *mut Object = msg_send![pdf_doc, initWithURL: ns_url];

            if pdf_doc.is_null() {
                return Err(PrintError::PrintError(
                    "Failed to create PDFDocument".to_string(),
                ));
            }

            let pdf_view: *mut Object = msg_send![class!(PDFView), alloc];
            let pdf_view: *mut Object = msg_send![pdf_view, init];

            if pdf_view.is_null() {
                let _: () = msg_send![pdf_doc, release];
                return Err(PrintError::PrintError(
                    "Failed to create PDFView".to_string(),
                ));
            }

            let _: () = msg_send![pdf_view, setDocument: pdf_doc];

            let print_info: *mut Object = msg_send![class!(NSPrintInfo), sharedPrintInfo];
            let print_op: *mut Object = msg_send![class!(NSPrintOperation), printOperationWithView: pdf_view
                                                                            printInfo: print_info];

            if print_op.is_null() {
                let _: () = msg_send![pdf_view, release];
                let _: () = msg_send![pdf_doc, release];
                return Err(PrintError::PrintError(
                    "Failed to create NSPrintOperation".to_string(),
                ));
            }

            let _: () = msg_send![print_op, runOperation];

            let _: () = msg_send![pdf_view, release];
            let _: () = msg_send![pdf_doc, release];
        }

        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    pub fn show_print_dialog(_pdf_path: &Path) -> Result<()> {
        Err(PrintError::PrintError(
            "Native print dialog not supported on this platform".to_string(),
        ))
    }
}

pub use MacOSPrinter as PlatformPrinter;
