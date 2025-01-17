use crate::{plugin::*, stream::*};

use std::ffi::CStr;

pub const CLAP_EXT_STATE_CONTEXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.state-context.draft/1\0") };

pub const CLAP_STATE_CONTEXT_FOR_DUPLICATE: clap_plugin_state_context_type = 1;
pub const CLAP_STATE_CONTEXT_FOR_PRESET: clap_plugin_state_context_type = 2;

pub type clap_plugin_state_context_type = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_state_context {
    pub save: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            stream: *const clap_ostream,
            context_type: clap_plugin_state_context_type,
        ) -> bool,
    >,
    pub load: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            stream: *const clap_istream,
            context_type: clap_plugin_state_context_type,
        ) -> bool,
    >,
}
