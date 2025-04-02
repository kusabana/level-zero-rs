use std::{ptr::addr_of_mut, u32};

use level_zero_sys::{
    _ze_driver_handle_t, ze_driver_handle_t, ze_init_driver_type_desc_t,
    ze_structure_type_t, zeInitDrivers,
};

fn main() {
    let mut count = 0u32;
    let mut drivers: [ze_driver_handle_t; 0] = [];
    let mut desc = ze_init_driver_type_desc_t {
        stype: ze_structure_type_t::ZE_STRUCTURE_TYPE_DRIVER_PROPERTIES,
        pNext: std::ptr::null(),
        flags: u32::MAX,
    };

    unsafe {
        let res = zeInitDrivers(
            addr_of_mut!(count),
            addr_of_mut!(drivers) as *mut *mut _ze_driver_handle_t,
            addr_of_mut!(desc),
        );
        dbg!(res);
        dbg!(count);
    }
}
