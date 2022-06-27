mod readarg;

use std::ffi::CString;
use std::ffi::CStr;

fn main()
{
    unsafe
    {
        /* Create a CString */
        let cstr = CString::new("greeting!").expect("CString::new() failed");
        let cstr2: &CStr = CStr::from_ptr(cstr.as_ptr());

        let _ra = readarg::read_args_t
        {
            count: 1,
            offset: 0,
            realcount: 10,
            buf: cstr2.as_ptr() as *mut u8
        };

        println!("cstr: {:?}, ra->buf: {:?}", cstr2, _ra.buf);
    };
}