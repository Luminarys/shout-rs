extern crate libc;

use libc::{c_char, c_uchar, c_int, c_uint, c_ushort, size_t, ssize_t};

pub enum Shout {}
pub enum ShoutMetadata {}

extern "C" {
    pub fn shout_init();
    pub fn shout_shutdown();
    pub fn shout_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> *const c_char;
    pub fn shout_new() -> *mut Shout;
    pub fn shout_free(shout: *mut Shout);

    pub fn shout_get_error(shout: *mut Shout) -> *const c_char;
    pub fn shout_get_errno(shout: *mut Shout) -> c_int;

    pub fn shout_get_connected(shout: *mut Shout) -> c_int;

    pub fn shout_set_host(shout: *mut Shout, host: *const c_char) -> c_int;
    pub fn shout_get_host(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_port(shout: *mut Shout, port: c_ushort) -> c_int;
    pub fn shout_get_port(shout: *mut Shout) -> c_ushort;

    pub fn shout_set_agent(shout: *mut Shout, agent: *const c_char) -> c_int;
    pub fn shout_get_agent(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_tls(shout: *mut Shout, mode: c_int) -> c_int;
    pub fn shout_get_tls(shout: *mut Shout) -> c_int;

    pub fn shout_set_ca_directory(shout: *mut Shout, directory: *const c_char) -> c_int;
    pub fn shout_get_ca_directory(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_ca_file(shout: *mut Shout, file: *const c_char) -> c_int;
    pub fn shout_get_ca_file(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_allowed_ciphers(shout: *mut Shout, ciphers: *const c_char) -> c_int;
    pub fn shout_get_allowed_ciphers(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_user(shout: *mut Shout, user: *const c_char) -> c_int;
    pub fn shout_get_user(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_password(shout: *mut Shout, password: *const c_char) -> c_int;
    pub fn shout_get_password(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_client_certificate(shout: *mut Shout, certificate: *const c_char) -> c_int;
    pub fn shout_get_client_certificate(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_mount(shout: *mut Shout, mount: *const c_char) -> c_int;
    pub fn shout_get_mount(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_dumpfile(shout: *mut Shout, dumpfile: *const c_char) -> c_int;
    pub fn shout_get_dumpfile(shout: *mut Shout) -> *const c_char;

    pub fn shout_set_audio_info(shout: *mut Shout, name: *const c_char, value: *const c_char) -> c_int;
    pub fn shout_get_audio_info(shout: *mut Shout, name: *const c_char) -> *const c_char;

    pub fn shout_set_meta(shout: *mut Shout, name: *const c_char, value: *const c_char) -> c_int;
    pub fn shout_get_meta(shout: *mut Shout, name: *const c_char) -> *const c_char;

    pub fn shout_set_public(shout: *mut Shout, make_public: c_uint) -> c_int;
    pub fn shout_get_public(shout: *mut Shout) -> c_uint;

    pub fn shout_set_format(shout: *mut Shout, format: c_uint) -> c_int;
    pub fn shout_get_format(shout: *mut Shout) -> c_uint;

    pub fn shout_set_protocol(shout: *mut Shout, protocol: c_uint) -> c_int;
    pub fn shout_get_protocol(shout: *mut Shout) -> c_uint;

    pub fn shout_set_nonblocking(shout: *mut Shout, protocol: c_uint) -> c_int;
    pub fn shout_get_nonblocking(shout: *mut Shout) -> c_uint;

    pub fn shout_open(shout: *mut Shout) -> c_int;
    pub fn shout_close(shout: *mut Shout) -> c_int;

    pub fn shout_send(shout: *mut Shout, data: *const c_uchar, len: size_t) -> ssize_t;
    pub fn shout_send_raw(shout: *mut Shout, data: *const c_uchar, len: size_t) -> ssize_t;

    pub fn shout_queue_len(shout: *mut Shout) -> ssize_t;

    pub fn shout_sync(shout: *mut Shout);

    pub fn shout_delay(shout: *mut Shout) -> c_int;

    pub fn shout_set_metadata(shout: *mut Shout) -> c_int;
    pub fn shout_metadata_new() -> *mut ShoutMetadata;
    pub fn shout_metadata_add(metadata: *mut ShoutMetadata, name: *const c_char, value: *const c_char) -> c_int;
}
