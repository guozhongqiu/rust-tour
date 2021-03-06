/* automatically generated by rust-bindgen 0.59.2 */

pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type size_t = ::std::os::raw::c_ulonglong;
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct read_args {
    pub count: u32,
    pub offset: u64,
    pub buf: *mut u8,
    pub realcount: u32,
}
#[test]
fn bindgen_test_layout_read_args() {
    assert_eq!(
        ::std::mem::size_of::<read_args>(),
        32usize,
        concat!("Size of: ", stringify!(read_args))
    );
    assert_eq!(
        ::std::mem::align_of::<read_args>(),
        8usize,
        concat!("Alignment of ", stringify!(read_args))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<read_args>())).count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(read_args),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<read_args>())).offset as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(read_args),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<read_args>())).buf as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(read_args),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<read_args>())).realcount as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(read_args),
            "::",
            stringify!(realcount)
        )
    );
}
pub type read_args_t = read_args;
extern "C" {
    pub fn fill_buf(ra: *mut read_args_t, size: u32) -> i32;
}
