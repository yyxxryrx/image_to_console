use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

static PID: LazyLock<u32> = LazyLock::new(std::process::id);
static COUNTER: AtomicU64 = AtomicU64::new(0);
static ID: LazyLock<u128> = LazyLock::new(rand::random);

#[cfg(target_os = "linux")]
pub static SHM: LazyLock<std::sync::Mutex<CleanShm>> =
    LazyLock::new(|| std::sync::Mutex::new(CleanShm::default()));

#[cfg(target_os = "linux")]
pub struct CleanShm {
    names: std::collections::VecDeque<String>,
    max_len: usize,
}

#[cfg(target_os = "linux")]
impl Default for CleanShm {
    fn default() -> Self {
        Self {
            names: Default::default(),
            max_len: usize::MAX,
        }
    }
}

#[cfg(target_os = "linux")]
impl CleanShm {
    pub fn add(&mut self, name: String) {
        if self.names.len() >= self.max_len && self.max_len > 0 {
            unsafe {
                let name = self.names.pop_back().unwrap_unchecked();
                if let Ok(c_name) = std::ffi::CString::new(name) {
                    libc::shm_unlink(c_name.as_ptr());
                }
            }
        }
        self.names.push_back(name);
    }

    pub fn max_len(&self) -> usize {
        self.max_len
    }

    pub fn set_max_len(&mut self, new_leng: std::num::NonZeroUsize) {
        self.max_len = new_leng.get();
    }
}

#[cfg(target_os = "linux")]
impl Drop for CleanShm {
    fn drop(&mut self) {
        while let Some(name) = self.names.pop_back() {
            let name = std::ffi::CString::new(name).unwrap();
            unsafe {
                libc::shm_unlink(name.as_ptr());
            }
        }
    }
}

pub fn gen_shm_name() -> String {
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    let name = format!("/itcc-shm-{:x}-{seq:x}-{:x}", *PID, *ID);
    #[cfg(target_os = "linux")]
    SHM.lock()
        .unwrap_or_else(|e| e.into_inner())
        .add(name.clone());
    name
}
