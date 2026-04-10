use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Context, Result, bail};
use libloading::Library;
use solana_signature::Signature;

use crate::TxnDataFfi;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AuraInitConfig {
    pub mock_payload: u64,
}

/// # Safety
/// Both the .so and loader MUST be compiled with the same rustc version,
/// target, and global allocator. No stable ABI guarantee otherwise.
type FfiInit = unsafe extern "C" fn(AuraInitConfig) -> *mut Result<(), String>;
type FfiSend = for<'a> unsafe extern "C" fn(TxnDataFfi<'a>) -> Vec<Signature>;

const SYMBOL_INIT: &[u8] = b"aura_ffi_init\0";
const SYMBOL_SEND: &[u8] = b"aura_send_transaction\0";

static LIB: OnceLock<AuraSenderLib> = OnceLock::new();

pub fn load_aura_sender(
    path: impl AsRef<Path>,
    cfg: AuraInitConfig,
) -> Result<&'static AuraSenderLib> {
    if let Some(lib) = LIB.get() {
        return Ok(lib);
    }
    let lib = AuraSenderLib::load(&path)?;
    lib.init(cfg)?;
    let _ = LIB.set(lib);
    Ok(LIB.get().unwrap())
}

pub struct AuraSenderLib {
    _lib: Library,
    init_fn: FfiInit,
    send_fn: FfiSend,
}

impl AuraSenderLib {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let lib = unsafe { Library::new(path.as_ref()) }
            .with_context(|| format!("failed to load shared library {:?}", path.as_ref()))?;
        unsafe {
            Ok(Self {
                init_fn: *lib.get::<FfiInit>(SYMBOL_INIT)?,
                send_fn: *lib.get::<FfiSend>(SYMBOL_SEND)?,
                _lib: lib,
            })
        }
    }

    pub fn init(&self, cfg: AuraInitConfig) -> Result<()> {
        unsafe { take_result((self.init_fn)(cfg)) }
    }
    #[inline]
    pub fn send_transaction(&self, txn: TxnDataFfi) -> Vec<Signature> {
        unsafe { (self.send_fn)(txn) }
    }
}

/// Reclaim a `Box<Result<T, String>>` returned by the .so and convert to `anyhow::Result`.
unsafe fn take_result<T>(ptr: *mut Result<T, String>) -> Result<T> {
    if ptr.is_null() {
        bail!("ffi returned null");
    }
    unsafe { Box::from_raw(ptr).map_err(|e| anyhow::anyhow!(e)) }
}
