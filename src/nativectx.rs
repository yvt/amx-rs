//! Safely manages AMX's actiavtion state.
use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
};

use crate::nativeops::AmxOps;

/// Represents the current thread's AMX context.
pub struct AmxCtx {
    ops: AmxOps<'static>,
}

/// The error type for [`AmxCtx::new`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NewAmxCtxError {
    /// The current thread already has an active `AmxCtx`.
    AlreadyActive,
    /// AMX is not supported by the target system.
    ///
    /// TODO: Report this error if it's unsupported
    Unsupported,
}

thread_local! {
    static CTX_ACTIVE: Cell<bool> = Cell::new(false);
}

impl AmxCtx {
    /// Construct a brand new instance of `AmxCtx` by enabling AMX for the
    /// current thread.
    pub fn new() -> Result<Self, NewAmxCtxError> {
        if CTX_ACTIVE.with(|x| x.get()) {
            Err(NewAmxCtxError::AlreadyActive)
        } else {
            // TODO: Don't assume AMX is always supported

            // Enable AMX for the current thread
            // Safety: AMX is supported
            unsafe { crate::nativeops::set() };

            Ok(Self {
                // Safety: AMX is supported
                ops: unsafe { AmxOps::new() },
            })
        }
    }
}

impl Drop for AmxCtx {
    fn drop(&mut self) {
        // Disable AMX for the current thread
        // Safety: AMX is supported
        unsafe { crate::nativeops::clr() };

        CTX_ACTIVE.with(|x| x.set(false));
    }
}

impl Deref for AmxCtx {
    type Target = AmxOps<'static>;

    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

impl DerefMut for AmxCtx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ops
    }
}
