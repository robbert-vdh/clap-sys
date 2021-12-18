use clap_sys::{events::*, ext::params::*, host::*, id::*, plugin::*, process::*, version::*};

use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::ptr;

mod params {
    use super::*;

    pub unsafe extern "C" fn count(_plugin: *const clap_plugin) -> u32 {
        0
    }

    pub unsafe extern "C" fn get_info(
        _plugin: *const clap_plugin,
        _param_index: i32,
        _param_info: *mut clap_param_info,
    ) -> bool {
        false
    }

    pub unsafe extern "C" fn get_value(
        _plugin: *const clap_plugin,
        _param_id: clap_id,
        _value: *mut f64,
    ) -> bool {
        false
    }

    pub unsafe extern "C" fn value_to_text(
        _plugin: *const clap_plugin,
        _param_id: clap_id,
        _value: f64,
        _display: *mut c_char,
        _size: u32,
    ) -> bool {
        false
    }

    pub unsafe extern "C" fn text_to_value(
        _plugin: *const clap_plugin,
        _param_id: clap_id,
        _display: *const c_char,
        _value: *mut f64,
    ) -> bool {
        false
    }

    pub unsafe extern "C" fn flush(
        _plugin: *const clap_plugin,
        _input_parameter_changes: *const clap_event_list,
        _output_parameter_changes: *const clap_event_list,
    ) {
    }
}

static PLUGIN_PARAMS: clap_plugin_params = clap_plugin_params {
    count: params::count,
    get_info: params::get_info,
    get_value: params::get_value,
    value_to_text: params::value_to_text,
    text_to_value: params::text_to_value,
    flush: params::flush,
};

mod plugin {
    use super::*;

    pub unsafe extern "C" fn init(_plugin: *const clap_plugin) -> bool {
        true
    }

    pub unsafe extern "C" fn destroy(plugin: *const clap_plugin) {
        drop(Box::from_raw(plugin as *mut clap_plugin));
    }

    pub unsafe extern "C" fn activate(
        _plugin: *const clap_plugin,
        _sample_rate: f64,
        _min_frames_count: u32,
        _max_frames_count: u32,
    ) {
    }

    pub unsafe extern "C" fn deactivate(_plugin: *const clap_plugin) {}

    pub unsafe extern "C" fn start_processing(_plugin: *const clap_plugin) -> bool {
        true
    }

    pub unsafe extern "C" fn stop_processing(_plugin: *const clap_plugin) {}

    pub unsafe extern "C" fn process(
        _plugin: *const clap_plugin,
        _process: *const clap_process,
    ) -> clap_process_status {
        CLAP_PROCESS_CONTINUE
    }

    pub unsafe extern "C" fn get_extension(
        _plugin: *const clap_plugin,
        id: *const c_char,
    ) -> *const c_void {
        if CStr::from_ptr(id) == CStr::from_ptr(CLAP_EXT_PARAMS) {
            &PLUGIN_PARAMS as *const clap_plugin_params as *const c_void
        } else {
            ptr::null()
        }
    }

    pub unsafe extern "C" fn on_main_thread(_plugin: *const clap_plugin) {}
}

static PLUGIN_DESCRIPTOR: clap_plugin_descriptor = clap_plugin_descriptor {
    clap_version: CLAP_VERSION,
    id: b"gain\0".as_ptr() as *const c_char,
    name: b"\0".as_ptr() as *const c_char,
    vendor: b"\0".as_ptr() as *const c_char,
    url: b"\0".as_ptr() as *const c_char,
    manual_url: b"\0".as_ptr() as *const c_char,
    support_url: b"\0".as_ptr() as *const c_char,
    version: b"\0".as_ptr() as *const c_char,
    description: b"\0".as_ptr() as *const c_char,
    keywords: b"\0".as_ptr() as *const c_char,
    plugin_type: CLAP_PLUGIN_AUDIO_EFFECT as u64,
};

mod entry {
    use super::*;

    pub unsafe extern "C" fn init(_plugin_path: *const c_char) {}

    pub unsafe extern "C" fn deinit() {}

    pub unsafe extern "C" fn get_plugin_count() -> u32 {
        1
    }

    pub unsafe extern "C" fn get_plugin_descriptor(index: u32) -> *const clap_plugin_descriptor {
        match index {
            0 => &PLUGIN_DESCRIPTOR,
            _ => ptr::null(),
        }
    }

    pub unsafe extern "C" fn create_plugin(
        _host: *const clap_host,
        plugin_id: *const c_char,
    ) -> *const clap_plugin {
        if CStr::from_ptr(plugin_id) == CStr::from_ptr(PLUGIN_DESCRIPTOR.id) {
            Box::into_raw(Box::new(clap_plugin {
                desc: &PLUGIN_DESCRIPTOR,
                plugin_data: ptr::null_mut(),
                init: plugin::init,
                destroy: plugin::destroy,
                activate: plugin::activate,
                deactivate: plugin::deactivate,
                start_processing: plugin::start_processing,
                stop_processing: plugin::stop_processing,
                process: plugin::process,
                get_extension: plugin::get_extension,
                on_main_thread: plugin::on_main_thread,
            }))
        } else {
            ptr::null()
        }
    }

    pub unsafe extern "C" fn get_invalidation_source_count() -> u32 {
        0
    }

    pub unsafe extern "C" fn get_invalidation_source(
        _index: u32,
    ) -> *const clap_plugin_invalidation_source {
        ptr::null()
    }

    pub unsafe extern "C" fn refresh() {}
}

#[allow(non_upper_case_globals)]
#[no_mangle]
static clap_plugin_entry: clap_plugin_entry = clap_plugin_entry {
    clap_version: CLAP_VERSION,
    init: entry::init,
    deinit: entry::deinit,
    get_plugin_count: entry::get_plugin_count,
    get_plugin_descriptor: entry::get_plugin_descriptor,
    create_plugin: entry::create_plugin,
    get_invalidation_source_count: entry::get_invalidation_source_count,
    get_invalidation_source: entry::get_invalidation_source,
    refresh: entry::refresh,
};