pub const CODES_MISSING_DOUBLE: libc::c_double = -1e+100;

#[repr(C)]
pub struct CodesHandle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CodesContext {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BufrKeysIter {
    _private: [u8; 0],
}

#[link(name = "eccodes")]
extern "C" {
    pub fn codes_handle_new_from_message(
        c: *mut CodesContext,
        data: *const u8,
        data_len: libc::size_t,
    ) -> *mut CodesHandle;

    pub fn codes_handle_delete(h: *mut CodesHandle) -> libc::c_int;

    pub fn codes_get_long(
        h: *mut CodesHandle,
        key: *const libc::c_char,
        value: *mut libc::c_long,
    ) -> libc::c_int;

    pub fn codes_set_long(
        h: *mut CodesHandle,
        key: *const libc::c_char,
        value: libc::c_long,
    ) -> libc::c_int;

    pub fn codes_bufr_keys_iterator_new(
        h: *mut CodesHandle,
        filter_flags: libc::c_ulong,
    ) -> *mut BufrKeysIter;

    pub fn codes_bufr_keys_iterator_delete(kiter: *mut BufrKeysIter) -> libc::c_int;

    pub fn codes_bufr_keys_iterator_next(kiter: *mut BufrKeysIter) -> libc::c_int;

    pub fn codes_bufr_keys_iterator_get_name(kiter: *mut BufrKeysIter) -> *const libc::c_char;

    pub fn codes_get_size(
        h: *mut CodesHandle,
        key: *const libc::c_char,
        value: *mut libc::size_t,
    ) -> libc::c_int;

    pub fn codes_get_double(
        h: *mut CodesHandle,
        key: *const libc::c_char,
        value: *mut libc::c_double,
    ) -> libc::c_int;

    pub fn codes_get_double_array(
        h: *mut CodesHandle,
        key: *const libc::c_char,
        vals: *mut libc::c_double,
        length: *mut libc::size_t,
    ) -> libc::c_int;
}
