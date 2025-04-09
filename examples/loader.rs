use std::mem::MaybeUninit;

use level_zero::Loader;
use level_zero_sys::{
    ze_driver_handle_t, ze_init_driver_type_desc_t, ze_result_t, ze_structure_type_t, zeInitDrivers,
};

fn main() {
    let mut driver_count = 1;
    let mut driver_handles: [ze_driver_handle_t; 1] =
        unsafe { MaybeUninit::zeroed().assume_init() };

    let mut descriptor = ze_init_driver_type_desc_t {
        stype: ze_structure_type_t::ZE_STRUCTURE_TYPE_DRIVER_PROPERTIES,
        flags: u32::MAX,
        pNext: std::ptr::null(),
    };

    assert_eq!(
        unsafe {
            zeInitDrivers(
                &raw mut driver_count,
                driver_handles.as_mut_ptr(),
                &raw mut descriptor,
            )
        },
        ze_result_t::ZE_RESULT_SUCCESS
    );
    assert!(driver_count > 0);

    Loader::get_components()
        .expect("Failed to get loader components")
        .iter()
        .for_each(|component| println!("Component: {component}"));
}
