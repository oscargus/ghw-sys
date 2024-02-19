use std::ffi::CString;
use std::path::PathBuf;

use ghw_sys::{
    ghw_close, ghw_disp_hie, ghw_disp_types, ghw_disp_values, ghw_handler, ghw_open, ghw_read_base,
};

fn ghw_asset(rel_path: &str) -> PathBuf {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.pop();
    path.push("assets");
    path.push(rel_path);
    path
}

#[test]
pub fn test_read_file() {
    let ghw_path = ghw_asset("adder_tb.ghw");
    let path = CString::new(ghw_path.to_str().unwrap()).unwrap();
    let ptr = path.as_ptr();
    unsafe {
        let mut handle: std::mem::MaybeUninit<ghw_handler> = std::mem::MaybeUninit::uninit();
        ghw_open(handle.as_mut_ptr(), ptr);
        (*handle.as_mut_ptr()).flag_verbose = 1;
        let _res = ghw_read_base(handle.as_mut_ptr());
        eprintln!("Display values");
        ghw_disp_values(handle.as_mut_ptr());
        eprintln!("Display types");
        ghw_disp_types(handle.as_mut_ptr());
        (*handle.as_mut_ptr()).flag_full_names = 1;
        eprintln!("Display hierarchy");
        ghw_disp_hie(handle.as_mut_ptr(), (*handle.as_mut_ptr()).hie);
        assert_eq!( (*handle.as_mut_ptr()).nbr_str, 14);
        assert_eq!( (*handle.as_mut_ptr()).nbr_sigs, 6);
        assert_eq!( (*handle.as_mut_ptr()).nbr_types, 1);
        ghw_close(handle.as_mut_ptr());
    }
}
