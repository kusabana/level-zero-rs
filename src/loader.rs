use crate::error::Result;
use level_zero_sys::{
    ze_result_t, zel_component_version_t, zelDisableTracingLayer, zelEnableTracingLayer,
    zelLoaderGetVersions,
};
use std::{fmt::Display, mem::MaybeUninit};

pub struct LoaderComponent(zel_component_version_t);

impl LoaderComponent {
    pub fn name(&self) -> &'_ str {
        let utf8: &[u8; 64] = unsafe { std::mem::transmute(&self.0.component_name) };
        std::str::from_utf8(utf8).unwrap_or("<none>")
    }

    pub fn version(&self) -> (i32, i32, i32) {
        (
            self.0.component_lib_version.major,
            self.0.component_lib_version.minor,
            self.0.component_lib_version.patch,
        )
    }
}

impl Display for LoaderComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name();
        let (major, minor, patch) = self.version();
        write!(f, "{name} v{major}.{minor}.{patch}")
    }
}

pub struct Loader;

impl Loader {
    /// Get list of loader components
    pub fn get_components() -> Result<Vec<LoaderComponent>> {
        let mut component_count = 0;

        let result =
            unsafe { zelLoaderGetVersions(&raw mut component_count, std::ptr::null_mut()) };
        if result != ze_result_t::ZE_RESULT_SUCCESS {
            return Err(result.into());
        }
        
        let mut components: Vec<zel_component_version_t> =
            vec![unsafe { MaybeUninit::zeroed().assume_init() }; component_count];
        let result =
            unsafe { zelLoaderGetVersions(&raw mut component_count, components.as_mut_ptr()) };
        if result != ze_result_t::ZE_RESULT_SUCCESS {
            return Err(result.into());
        }

        Ok(components.into_iter().map(LoaderComponent).collect())
    }

    /// Increments tracing layer reference count
    pub fn enable_tracing_layer() -> Result<()> {
        let result = unsafe { zelEnableTracingLayer() };
        if result != ze_result_t::ZE_RESULT_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }

    /// Decrements tracing layer reference count
    pub fn disable_tracing_layer() -> Result<()> {
        let result = unsafe { zelDisableTracingLayer() };
        if result != ze_result_t::ZE_RESULT_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }
}
