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

    /// Sets the format of the stream, the usage, and the used codecs.
    ///
    /// The format must be one of the supported format constants listed in
    /// ShoutFormat.  The default is `Ogg`.
    ///
    /// The usage parameter is a bit-wise or-ed set of usages from the usage
    /// constants listed in ShoutUsage.  The default is `Unknown`.
    ///
    /// The codecs parameter is used for codec pinning.  In codec pinning mode
    /// only the listed codecs are allowed to be contained in the stream.  This
    /// helps listening software with playback.  This is not yet supported and
    /// must be set to NULL meaning no pinning.  A value of NULL will also be
    /// supported by future versions.
    ///
    /// *Note*: `ShoutFormat::WebmAudio` isn't accepted by this function.  Use
    /// `ShoutFormat::Webm` and `ShoutUsage::Audio`.
    ///
    /// *Note*: This isn't exposed by the higher-level library, which assumes
    /// the usage will always be `Audio`.
    pub fn shout_set_content_format(shout: *mut Shout, format: c_uint, usage: c_uint, codecs: *const c_char) -> c_int;
    /// Returns the content format parameters as set by
    /// `shout_set_content_format`.
    pub fn shout_get_content_format(shout: *mut Shout, format: *mut c_uint, usage: *mut c_uint, codecs: *mut *const c_char) -> c_int;

    pub fn shout_set_protocol(shout: *mut Shout, protocol: c_uint) -> c_int;
    pub fn shout_get_protocol(shout: *mut Shout) -> c_uint;

    pub fn shout_set_nonblocking(shout: *mut Shout, protocol: c_uint) -> c_int;
    pub fn shout_get_nonblocking(shout: *mut Shout) -> c_uint;

    pub fn shout_open(shout: *mut Shout) -> c_int;
    pub fn shout_close(shout: *mut Shout) -> c_int;

    pub fn shout_send(shout: *mut Shout, data: *const c_uchar, len: size_t) -> c_int;

    #[deprecated(since = "2.4.5")]
    pub fn shout_send_raw(shout: *mut Shout, data: *const c_uchar, len: size_t) -> ssize_t;

    pub fn shout_queuelen(shout: *mut Shout) -> ssize_t;

    pub fn shout_sync(shout: *mut Shout);

    pub fn shout_delay(shout: *mut Shout) -> c_int;

    pub fn shout_set_metadata(shout: *mut Shout, metadata: *mut ShoutMetadata) -> c_int;
    pub fn shout_metadata_new() -> *mut ShoutMetadata;
    pub fn shout_metadata_free(metadata: *mut ShoutMetadata);
    pub fn shout_metadata_add(metadata: *mut ShoutMetadata, name: *const c_char, value: *const c_char) -> c_int;

    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_set_meta instead."
    )]
    pub fn shout_set_name(shout: *mut Shout, name: *const c_char) -> c_int;
    pub fn shout_get_name(shout: *mut Shout) -> *const c_char;

    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_set_content_format instead."
    )]
    pub fn shout_set_format(shout: *mut Shout, format: c_uint) -> c_int;
    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_get_content_format instead."
    )]
    pub fn shout_get_format(shout: *mut Shout) -> c_uint;

    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_set_meta instead."
    )]
    pub fn shout_set_url(shout: *mut Shout, cur: *const c_char) -> c_int;
    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_get_meta instead."
    )]
    pub fn shout_get_url(shout: *mut Shout) -> *const c_char;

    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_set_meta instead."
    )]
    pub fn shout_set_genre(shout: *mut Shout, genre: *const c_char) -> c_int;
    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_get_meta instead."
    )]
    pub fn shout_get_genre(shout: *mut Shout) -> *const c_char;

    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_set_meta instead."
    )]
    pub fn shout_set_description(shout: *mut Shout, description: *const c_char) -> c_int;
    #[deprecated(
        since = "2.4.5",
        note = "Please use shout_get_meta instead."
    )]
    pub fn shout_get_description(shout: *mut Shout) -> *const c_char;
}
