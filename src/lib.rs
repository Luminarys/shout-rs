extern crate shout_sys as sys;

use std::ffi::{CString, NulError};

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
static GLOBAL_INSTANCE_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

/// Type representing the return of a call to a libshout function.
/// The Success value should never be returned as an error by this library.
#[derive(Copy, Clone, Debug)]
pub enum ShoutErr {
    /// No error
    Success = 0,
    /// Nonsensical arguments
    Insane = -1,
    /// Couldn't connect
    NoConnect = -2,
    /// Login failed
    NoLogin = -3,
    /// Socket error
    Socket = -4,
    /// Out of memory
    Malloc = -5,
    Metadata = -6,
    /// Cannot set parameter while connected
    Connected = -7,
    /// Not connected
    Unconnected = -8,
    /// This libshout version doesn't support the requested operation
    Unsupported = -9,
    /// The socket is busy
    Busy = -10,
    /// TLS requested but not supported by the peer
    NoTLS = -11,
    /// TLS connection cannot be established due to bad certificate
    TLSBadCert = -12,
    /// Retry last operation
    Retry = -13,
}

impl ShoutErr {
    fn new(i: i32) -> ShoutErr {
        match i {
            0 => ShoutErr::Success,
            -1 => ShoutErr::Insane,
            -2 => ShoutErr::NoConnect,
            -3 => ShoutErr::NoLogin,
            -4 => ShoutErr::Socket,
            -5 => ShoutErr::Malloc,
            -6 => ShoutErr::Metadata,
            -7 => ShoutErr::Connected,
            -8 => ShoutErr::Unconnected,
            -9 => ShoutErr::Unsupported,
            -10 => ShoutErr::Busy,
            -11 => ShoutErr::NoTLS,
            -12 => ShoutErr::TLSBadCert,
            -13 => ShoutErr::Retry,
            _ => unreachable!(),
        }
    }

    pub fn success(&self) -> bool {
        match *self {
            ShoutErr::Success => true,
            _ => false,
        }
    }
}

/// Type representing a TLS mode to connect to a host with
#[derive(Copy, Clone, Debug)]
pub enum ShoutTLS {
    /// Do not use TLS at all
    Disabled = 0,
    /// Autodetect which TLS mode to use if any
    Auto = 1,
    /// Like Auto, but does not allow plain connections
    AutoNoPlain = 2,
    /// USE TLS for transport layer like HTTPS(RFC2818) does
    RFC2818 = 11,
    /// USE TLS via HTTP Upgrade:-header (RFC2817)
    RFC2817 = 12,
}

/// Type representing the format of data to be streamed to the host is
#[derive(Copy, Clone, Debug)]
pub enum ShoutFormat {
    /// application/ogg
    Ogg = 0,
    /// audio/mpeg
    MP3 = 1,
    /// video/webm
    Webm = 2,
    /// audio/webm audio only
    WebmAudio = 3,
}

/// Type representing the protocol to use for libshout
#[derive(Copy, Clone, Debug)]
pub enum ShoutProtocol {
    HTTP = 0,
    XAudioCast = 1,
    Icy = 2,
    RoarAudio = 3,
}

pub static SHOUT_META_NAME: &'static str = "name";
pub static SHOUT_META_URL: &'static str = "url";
pub static SHOUT_META_GENRE: &'static str = "genre";
pub static SHOUT_META_DESCRIPTION: &'static str = "description";
pub static SHOUT_META_IRC: &'static str = "irc";
pub static SHOUT_META_AIM: &'static str = "aim";
pub static SHOUT_META_ICQ: &'static str = "icq";

/// Type representing a meta value used in setting up the connection with the host.
#[derive(Clone, Debug)]
pub enum ShoutMeta {
    Name(String),
    Url(String),
    Genre(String),
    Description(String),
    IRC(String),
    AIM(String),
    ICQ(String),
}

pub static SHOUT_AI_BITRATE: &'static str = "bitrate";
pub static SHOUT_AI_SAMPLERATE: &'static str = "samplerate";
pub static SHOUT_AI_CHANNELS: &'static str = "channels";
pub static SHOUT_AI_QUALITY: &'static str = "quality";

/// Type representing information about the audio data to be sent to the host
#[derive(Clone, Debug)]
pub enum ShoutAudioInfo {
    BitRate(String),
    SampleRate(String),
    Channels(String),
    Quality(String),
}

/// Type representing an error resulting from either libshout, or processing data to be sent to
/// libshout
#[derive(Debug)]
pub enum ShoutConnError {
    ShoutError(ShoutErr),
    NulError(NulError),
}

macro_rules! shout_conn_err {
    ($func:expr) => (
        {
            let i = $func;
            if i != 0 {
                return Err(ShoutConnError::ShoutError(ShoutErr::new(i)));
            }
        }
    );
}

/// A shout connection builder. All desired values should be set in this before it is built into a
/// `ShoutConn`. All validation of parameters and FFI calls happen on build.
#[derive(Default)]
pub struct ShoutConnBuilder {
    host: Option<String>,
    port: Option<u16>,
    agent: Option<String>,
    tls: Option<ShoutTLS>,
    ca_directory: Option<String>,
    ca_file: Option<String>,
    allowed_ciphers: Option<String>,
    user: Option<String>,
    password: Option<String>,
    client_cert: Option<String>,
    mount: Option<String>,
    dumpfile: Option<String>,
    audio_info: Vec<ShoutAudioInfo>,
    meta: Vec<ShoutMeta>,
    public: Option<u32>,
    format: Option<ShoutFormat>,
    protocol: Option<ShoutProtocol>,
    nonblocking: Option<u32>,
}

impl ShoutConnBuilder {
    pub fn new() -> ShoutConnBuilder {
        Default::default()
    }

    pub fn add_audio_info(mut self, audio_info: ShoutAudioInfo) -> ShoutConnBuilder {
        self.audio_info.push(audio_info);
        self
    }

    pub fn add_meta(mut self, meta: ShoutMeta) -> ShoutConnBuilder {
        self.meta.push(meta);
        self
    }

    pub fn build(self) -> Result<ShoutConn, ShoutConnError> {
        macro_rules! shout_set_string {
            ($field:ident, $shout:ident, $func:path) => (
                {
                    if let Some(val) = self.$field {
                        match CString::new(val) {
                            Ok(cstr) => {
                                shout_conn_err!($func($shout, cstr.as_ptr()));
                            }
                            Err(n) => {
                                return Err(ShoutConnError::NulError(n));
                            }
                        }
                    }
                }
            );
        }

        macro_rules! shout_set_kv {
            ($field:ident, $val:ident, $shout:ident, $func:path) => (
                {
                    let k = CString::new($field).unwrap();
                    let v = CString::new($val).unwrap();
                    shout_conn_err!($func($shout, k.as_ptr(), v.as_ptr()))
                }
            );
        }

        unsafe {
            let instances = GLOBAL_INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst);
            if instances == 0 {
                sys::shout_init();
            }
            let shout = sys::shout_new();

            shout_set_string!(host, shout, sys::shout_set_host);

            if let Some(port) = self.port {
                shout_conn_err!(sys::shout_set_port(shout, port));
            }

            shout_set_string!(agent, shout, sys::shout_set_agent);

            match self.tls {
                Some(ShoutTLS::Disabled) => {
                    shout_conn_err!(sys::shout_set_tls(shout, 0));
                }
                Some(ShoutTLS::Auto) => {
                    shout_conn_err!(sys::shout_set_tls(shout, 1));
                }
                Some(ShoutTLS::AutoNoPlain) => {
                    shout_conn_err!(sys::shout_set_tls(shout, 2));
                }
                Some(ShoutTLS::RFC2818) => {
                    shout_conn_err!(sys::shout_set_tls(shout, 11));
                }
                Some(ShoutTLS::RFC2817) => {
                    shout_conn_err!(sys::shout_set_tls(shout, 12));
                }
                None => {}
            }

            shout_set_string!(ca_directory, shout, sys::shout_set_ca_directory);
            shout_set_string!(ca_file, shout, sys::shout_set_ca_file);
            shout_set_string!(allowed_ciphers, shout, sys::shout_set_allowed_ciphers);
            shout_set_string!(user, shout, sys::shout_set_user);
            shout_set_string!(password, shout, sys::shout_set_password);
            shout_set_string!(client_cert, shout, sys::shout_set_client_certificate);
            shout_set_string!(mount, shout, sys::shout_set_mount);
            shout_set_string!(dumpfile, shout, sys::shout_set_dumpfile);

            if let Some(public) = self.public {
                shout_conn_err!(sys::shout_set_public(shout, public));
            }

            match self.format {
                Some(ShoutFormat::Ogg) => {
                    shout_conn_err!(sys::shout_set_format(shout, 0));
                }
                Some(ShoutFormat::MP3) => {
                    shout_conn_err!(sys::shout_set_format(shout, 1));
                }
                Some(ShoutFormat::Webm) => {
                    shout_conn_err!(sys::shout_set_format(shout, 2));
                }
                Some(ShoutFormat::WebmAudio) => {
                    shout_conn_err!(sys::shout_set_format(shout, 3));
                }
                None => {}
            }

            match self.protocol {
                Some(ShoutProtocol::HTTP) => {
                    shout_conn_err!(sys::shout_set_protocol(shout, 0));
                }
                Some(ShoutProtocol::XAudioCast) => {
                    shout_conn_err!(sys::shout_set_protocol(shout, 1));
                }
                Some(ShoutProtocol::Icy) => {
                    shout_conn_err!(sys::shout_set_protocol(shout, 2));
                }
                Some(ShoutProtocol::RoarAudio) => {
                    shout_conn_err!(sys::shout_set_protocol(shout, 3));
                }
                None => {}
            }

            if let Some(nonblocking) = self.nonblocking {
                shout_conn_err!(sys::shout_set_nonblocking(shout, nonblocking));
            }

            for ai in self.audio_info {
                match ai {
                    ShoutAudioInfo::BitRate(val) => {
                        shout_set_kv!(SHOUT_AI_BITRATE, val, shout, sys::shout_set_audio_info);
                    }
                    ShoutAudioInfo::SampleRate(val) => {
                        shout_set_kv!(SHOUT_AI_SAMPLERATE, val, shout, sys::shout_set_audio_info);
                    }
                    ShoutAudioInfo::Channels(val) => {
                        shout_set_kv!(SHOUT_AI_CHANNELS, val, shout, sys::shout_set_audio_info);
                    }
                    ShoutAudioInfo::Quality(val) => {
                        shout_set_kv!(SHOUT_AI_QUALITY, val, shout, sys::shout_set_audio_info);
                    }
                }
            }

            for meta in self.meta {
                match meta {
                    ShoutMeta::Name(val) => {
                        shout_set_kv!(SHOUT_META_NAME, val, shout, sys::shout_set_meta);
                    }
                    ShoutMeta::Url(val) => {
                        shout_set_kv!(SHOUT_META_URL, val, shout, sys::shout_set_meta);
                    }
                    ShoutMeta::Genre(val) => {
                        shout_set_kv!(SHOUT_META_GENRE, val, shout, sys::shout_set_meta);
                    }
                    ShoutMeta::Description(val) => {
                        shout_set_kv!(SHOUT_META_DESCRIPTION, val, shout, sys::shout_set_meta);
                    }
                    ShoutMeta::IRC(val) => {
                        shout_set_kv!(SHOUT_META_IRC, val, shout, sys::shout_set_meta);
                    }
                    ShoutMeta::AIM(val) => {
                        shout_set_kv!(SHOUT_META_AIM, val, shout, sys::shout_set_meta);
                    }
                    ShoutMeta::ICQ(val) => {
                        shout_set_kv!(SHOUT_META_ICQ, val, shout, sys::shout_set_meta);
                    }
                }
            }

            shout_conn_err!(sys::shout_open(shout));
            Ok(ShoutConn { shout: shout })
        }
    }
}

macro_rules! default_build {
    ($struct_type:ty, $(($field:ident,$field_type:ty)),+) => (
        impl $struct_type {
            $(
                pub fn $field(mut self, $field: $field_type) -> $struct_type {
                    self.$field = Some($field);
                    self
                }
            )+
        }
    );
}

default_build!(ShoutConnBuilder,
               (host, String),
               (port, u16),
               (agent, String),
               (tls, ShoutTLS),
               (ca_directory, String),
               (ca_file, String),
               (allowed_ciphers, String),
               (user, String),
               (password, String),
               (client_cert, String),
               (mount, String),
               (dumpfile, String),
               (public, u32),
               (format, ShoutFormat),
               (protocol, ShoutProtocol),
               (nonblocking, u32));

/// Struct representing a metadata dict to be used by the shout connection
pub struct ShoutMetadata {
    metadata: *mut sys::ShoutMetadata,
}

impl ShoutMetadata {
    pub fn new() -> ShoutMetadata {
        unsafe { ShoutMetadata { metadata: sys::shout_metadata_new() } }
    }

    /// Adds a parameter into the metadata structure.
    pub fn add(&mut self, name: String, value: String) -> Result<(), ShoutConnError> {
        match (CString::new(name), CString::new(value)) {
            (Ok(n), Ok(v)) => {
                unsafe {
                    shout_conn_err!(sys::shout_metadata_add(self.metadata, n.as_ptr(), v.as_ptr()));
                }
                Ok(())
            }
            (Err(e), _) => Err(ShoutConnError::NulError(e)),
            (_, Err(e)) => Err(ShoutConnError::NulError(e)),
        }
    }
}

impl Drop for ShoutMetadata {
    fn drop(&mut self) {
        unsafe {
            sys::shout_metadata_free(self.metadata);
        }
    }
}

pub struct ShoutConn {
    shout: *mut sys::Shout,
}

impl ShoutConn {
    pub fn reconnect(&self) -> Result<(), ShoutConnError> {
        unsafe {
            sys::shout_close(self.shout);
            shout_conn_err!(sys::shout_open(self.shout));
        }
        return Ok(());
    }
    /// Sends data to the server, parsing it for format specific timing info.
    pub fn send(&self, data: Vec<u8>) -> Result<(), ShoutErr> {
        let len = data.len();
        let res = unsafe { sys::shout_send(self.shout, data.as_ptr() as *const u8, len) };
        if res != 0 {
            return Err(ShoutErr::new(res));
        } else {
            return Ok(())
        }
    }

    /// Sends unparsed data to the server. Do not use this unless you know what you're doing.
    /// Returns the number of bytes writter, or < 0 on error.
    pub fn send_raw(&self, data: Vec<u8>) -> isize {
        let len = data.len();
        unsafe { sys::shout_send_raw(self.shout, data.as_ptr() as *const u8, len) }
    }

    /// Returns the number of bytes on the write queue. Only makes sense in nonblocking mode.
    pub fn queue_len(&self) -> isize {
        unsafe { sys::shout_queuelen(self.shout) }
    }

    /// Sleeps the thread until the server requires more data
    pub fn sync(&self) {
        unsafe { sys::shout_sync(self.shout) }
    }

    /// Returns the amount of time the caller should wait before sending more data
    pub fn delay(&self) -> i32 {
        unsafe { sys::shout_delay(self.shout) }
    }

    /// Sets metadata for the host
    pub fn set_metadata(&self, metadata: ShoutMetadata) -> Result<(), ShoutConnError> {
        unsafe {
            shout_conn_err!(sys::shout_set_metadata(self.shout, metadata.metadata));
            Ok(())
        }
    }
}

impl Drop for ShoutConn {
    fn drop(&mut self) {
        unsafe {
            sys::shout_close(self.shout);
            sys::shout_free(self.shout);
            let instances = GLOBAL_INSTANCE_COUNT.fetch_sub(1, Ordering::SeqCst);
            if instances == 1 {
                sys::shout_shutdown();
            }
        }
    }
}

unsafe impl Send for ShoutConn { }

#[cfg(test)]
mod tests {
    use super::sys;
    #[test]
    fn it_works() {
        unsafe {
            sys::shout_init();
            let s = sys::shout_new();
            sys::shout_free(s);
            sys::shout_shutdown();
        }
    }
}
