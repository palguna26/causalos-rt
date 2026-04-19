use std::ptr::NonNull;
use windows_sys::Win32::Foundation::{INVALID_HANDLE_VALUE, HANDLE, CloseHandle};
use windows_sys::Win32::System::Memory::{
    CreateFileMappingW, MapViewOfFile, UnmapViewOfFile,
    PAGE_READWRITE, FILE_MAP_ALL_ACCESS, MEMORY_MAPPED_VIEW_ADDRESS
};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub struct MemorySegment {
    handle: HANDLE,
    ptr: NonNull<u8>,
    size: usize,
}

impl MemorySegment {
    pub fn create(name: &str, size: usize) -> Result<Self, String> {
        let name_wide: Vec<u16> = OsStr::new(name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let handle = CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                std::ptr::null(),
                PAGE_READWRITE,
                0,
                size as u32,
                name_wide.as_ptr(),
            );

            if handle == 0 {
                return Err(format!("Failed to create file mapping: {}", std::io::Error::last_os_error()));
            }

            let view_addr = MapViewOfFile(
                handle,
                FILE_MAP_ALL_ACCESS,
                0,
                0,
                size,
            );

            if view_addr.Value.is_null() {
                CloseHandle(handle);
                return Err(format!("Failed to map view of file: {}", std::io::Error::last_os_error()));
            }

            Ok(Self {
                handle,
                ptr: NonNull::new(view_addr.Value as *mut u8).unwrap(),
                size,
            })
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.size) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size) }
    }
}

impl Drop for MemorySegment {
    fn drop(&mut self) {
        unsafe {
            let view_addr = MEMORY_MAPPED_VIEW_ADDRESS { Value: self.ptr.as_ptr() as *mut _ };
            UnmapViewOfFile(view_addr);
            CloseHandle(self.handle);
        }
    }
}
