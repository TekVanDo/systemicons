use crate::Error;
use cocoa::{
    base::{id, nil},
    foundation::NSSize,
};
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::CString;
use std::slice;

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum NSBitmapImageFileType {
    NSBitmapImageFileTypePNG = 4,
}

pub fn get_icon(path: &str, size: u16) -> Result<Vec<u8>, Error> {
    let size: f64 = size.into();
    unsafe {
        // convert &str to NSString
        let ns_source_path: id =
            msg_send![class!(NSString), stringWithCString: CString::new(path).unwrap()];

        // get shared workspace
        let ns_workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];

        // get app icon
        let ns_image: id = msg_send![ns_workspace, iconForFile: ns_source_path];

        // set size
        let _: () = msg_send![ns_image, setSize: NSSize::new(size, size)];

        let cg_ref: id = msg_send![ns_image, CGImageForProposedRect:nil context:nil hints:nil];
        let ns_bitmap_image_ref: id = msg_send![class!(NSBitmapImageRep), alloc];
        let image_rep: id = msg_send![ns_bitmap_image_ref, initWithCGImage: cg_ref];
        let image_dimension: id = msg_send![ns_image, size];
        let _: () = msg_send![image_rep, setSize: image_dimension];

        let png_data: id = msg_send![image_rep, representationUsingType:NSBitmapImageFileType::NSBitmapImageFileTypePNG properties:nil];
        let ptr: *mut u8 = msg_send![png_data, bytes];
        let length: usize = msg_send![png_data, length];
        let bytes = slice::from_raw_parts(ptr, length).to_vec();
        let _: () = msg_send![image_rep, autorelease];

        Ok(bytes)
    }
}
