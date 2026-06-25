use std::{
    ffi::{CString, c_void},
    ptr::null_mut,
};

use libc::{
    MAP_FAILED, MAP_SHARED, O_CREAT, O_RDWR, PROT_READ, PROT_WRITE, close, ftruncate, memcpy, mmap,
    munmap, shm_open,
};

#[allow(unused)]
struct Shm {
    fd: i32,
    name: CString,
}

impl Shm {
    fn open(name: &str) -> error::ShmResult<Self> {
        let name = CString::new(name)?;
        let shm_fd = unsafe { shm_open(name.as_ptr(), O_CREAT | O_RDWR, 0o666) };
        if shm_fd == -1 {
            return Err(error::ShmError::OpenFailed);
        }
        Ok(Self { fd: shm_fd, name })
    }
}

impl Drop for Shm {
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}

#[allow(unused)]
pub struct SharedData {
    pub size: usize,
    ptr: *mut c_void,
    shm: Shm,
}

impl SharedData {
    pub fn new(name: &str, data: &[u8]) -> error::ShmResult<Self> {
        if data.is_empty() {
            return Err(error::ShmError::EmptyData);
        }
        let shm = Shm::open(name)?;
        unsafe {
            if ftruncate(shm.fd, data.len() as i64) == -1 {
                return Err(error::ShmError::TruncateFailed);
            }
        }

        let ptr = unsafe {
            let ptr = mmap(
                null_mut(),
                data.len(),
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                shm.fd,
                0,
            );

            if ptr == MAP_FAILED {
                return Err(error::ShmError::MapFailed);
            }

            memcpy(ptr, data.as_ptr() as *const c_void, data.len());
            ptr
        };

        Ok(Self {
            shm,
            ptr,
            size: data.len(),
        })
    }
}

impl Drop for SharedData {
    fn drop(&mut self) {
        unsafe {
            munmap(self.ptr, self.size);
        }
    }
}

pub mod error {
    use std::ffi::NulError;

    pub type ShmResult<T> = Result<T, ShmError>;

    #[derive(Debug)]
    pub enum ShmError {
        EmptyData,
        MapFailed,
        OpenFailed,
        TruncateFailed,
        ConvertError(NulError),
    }

    impl std::fmt::Display for ShmError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::EmptyData => write!(f, "Empty data"),
                Self::MapFailed => write!(f, "Map failed"),
                Self::TruncateFailed => write!(f, "Truncate failed"),
                Self::OpenFailed => write!(f, "Open failed"),
                Self::ConvertError(err) => write!(f, "ConvertError: {err}"),
            }
        }
    }

    impl std::error::Error for ShmError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Self::ConvertError(err) => Some(err),
                _ => None,
            }
        }
    }

    impl From<NulError> for ShmError {
        fn from(value: NulError) -> Self {
            Self::ConvertError(value)
        }
    }
}
