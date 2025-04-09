use std::{mem::MaybeUninit, ptr::addr_of_mut, u32};

use level_zero_sys::{
    ze_driver_handle_t, ze_init_driver_type_desc_t, ze_result_t,
    ze_structure_type_t, zeInitDrivers, zel_component_version_t, zelLoaderGetVersions,
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
                addr_of_mut!(driver_count),
                addr_of_mut!(driver_handles) as *mut ze_driver_handle_t,
                addr_of_mut!(descriptor),
            )
        },
        ze_result_t::ZE_RESULT_SUCCESS
    );
    assert!(driver_count > 0);

    let mut version_count = 0;
    assert_eq!(
        unsafe { zelLoaderGetVersions(addr_of_mut!(version_count), std::ptr::null_mut()) },
        ze_result_t::ZE_RESULT_SUCCESS
    );

    let mut versions: Vec<zel_component_version_t> =
        vec![unsafe { MaybeUninit::zeroed().assume_init() }; version_count];
    assert_eq!(
        unsafe { zelLoaderGetVersions(addr_of_mut!(version_count), versions.as_mut_ptr()) },
        ze_result_t::ZE_RESULT_SUCCESS
    );

    for (index, version) in versions.iter().enumerate() {
        println!("Version {index}");
        println!(
            "\tName: {}",
            std::str::from_utf8(unsafe { std::mem::transmute(&version.component_name[..]) })
                .unwrap_or_default()
        );
        println!(
            "\tVersion: {}.{}.{}",
            version.component_lib_version.major,
            version.component_lib_version.minor,
            version.component_lib_version.patch
        );
    }
}
