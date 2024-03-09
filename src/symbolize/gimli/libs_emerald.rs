// Emnerald, getting the only this-exe image
// We don't have the notion of shared libraries yet, so we
// just return the current executable as the only library.

use super::mystd::borrow::ToOwned;
use super::mystd::ffi::{CStr, OsStr};
use super::{Library, LibrarySegment, Vec};

pub(super) fn native_libraries() -> Vec<Library> {
    let process_meta = emerald_std::process::process_metadata();
    let name = super::mystd::env::current_exe().unwrap().as_os_str().to_owned();

    let this_exe = Library {
        name,
        segments: vec![LibrarySegment {
            stated_virtual_memory_address: process_meta.image_base,
            len: process_meta.image_size,
        }],
        bias: 0,
    };

    vec![this_exe]
}
