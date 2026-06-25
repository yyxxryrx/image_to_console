use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

static PID: LazyLock<u32> = LazyLock::new(std::process::id);
static COUNTER: AtomicU64 = AtomicU64::new(0);
static ID: LazyLock<u128> = LazyLock::new(rand::random);

pub fn gen_shm_name() -> String {
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("/itcc-shm-{:x}-{seq:x}-{:x}", *PID, *ID)
}
