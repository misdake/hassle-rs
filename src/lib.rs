#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(
    clippy::transmute_ptr_to_ptr, // Introduced by com-rs
    clippy::too_many_arguments, // We're wrapping and API outside of our control
    clippy::uninlined_format_args, // Unfavourable format; implies unneeded MSRV bump
)]

//! # Hassle
//!
//! This crate provides an FFI layer and idiomatic rust wrappers for the new [DirectXShaderCompiler](https://github.com/Microsoft/DirectXShaderCompiler) library.
//!
//! ## Simple example
//!
//! ```rust
//! use hassle_rs::compile_hlsl;
//!
//! let code = "
//!     Texture2D<float4> g_input    : register(t0, space0);
//!     RWTexture2D<float4> g_output : register(u0, space0);
//!
//!     [numthreads(8, 8, 1)]
//!     void copyCs(uint3 dispatchThreadId : SV_DispatchThreadID)
//!     {
//!         g_output[dispatchThreadId.xy] = g_input[dispatchThreadId.xy];
//!     }";
//!
//! let ir = compile_hlsl(
//!     "shader_filename.hlsl",
//!     code,
//!     Some("copyCs"),
//!     "cs_6_1",
//!     &vec!["-spirv"],
//!     &vec![
//!         ("MY_DEFINE", Some("Value")),
//!         ("OTHER_DEFINE", None)
//!     ],
//! );
//! ```

pub mod fake_sign;
pub mod ffi;
pub mod os;
pub mod utils;
pub mod wrapper;

pub mod intellisense;

pub use crate::ffi::*;
pub use crate::utils::{compile_hlsl, fake_sign_dxil_in_place, validate_dxil, HassleError, Result};
pub use crate::wrapper::*;
use std::mem::ManuallyDrop;

pub struct DxcPack {
    dxc: ManuallyDrop<Dxc>,
    compiler: ManuallyDrop<DxcCompiler>,
    library: ManuallyDrop<DxcLibrary>,
    dxil: ManuallyDrop<Dxil>,
    validator: ManuallyDrop<DxcValidator>,
}
impl Drop for DxcPack {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.validator);
            ManuallyDrop::drop(&mut self.dxil);
            ManuallyDrop::drop(&mut self.library);
            ManuallyDrop::drop(&mut self.compiler);
            ManuallyDrop::drop(&mut self.dxc);
        }
    }
}

impl DxcPack {
    pub fn create() -> Result<Self> {
        let dxc = Dxc::new(None)?;
        let compiler = dxc.create_compiler()?;
        let library = dxc.create_library()?;

        let dxil = Dxil::new(None)?;
        let validator = dxil.create_validator()?;

        Ok(Self {
            dxc: ManuallyDrop::new(dxc),
            compiler: ManuallyDrop::new(compiler),
            library: ManuallyDrop::new(library),
            dxil: ManuallyDrop::new(dxil),
            validator: ManuallyDrop::new(validator),
        })
    }

    pub fn compile_validate(
        &self,
        source_name: &str,
        shader_text: &str,
        entry_point: Option<&str>,
        target_profile: &str,
        args: &[&str],
        defines: &[(&str, Option<&str>)],
    ) -> Result<Vec<u8>> {
        use crate::utils::DefaultIncludeHandler;

        let blob = self
            .library
            .create_blob_with_encoding_from_str(shader_text)?;

        let result = self.compiler.compile(
            &blob,
            source_name,
            entry_point,
            target_profile,
            args,
            Some(&mut DefaultIncludeHandler {}),
            defines,
        );

        match result {
            Err(result) => {
                let error_blob = result.0.get_error_buffer()?;
                Err(HassleError::CompileError(
                    self.library.get_blob_as_string(&error_blob.into())?,
                ))
            }
            Ok(result) => {
                let data = result.get_result()?.to_vec();

                let blob_encoding = self.library.create_blob_with_encoding(&data)?;

                let result_blob = match self.validator.validate(blob_encoding.into()) {
                    Ok(blob) => blob,
                    Err(result) => {
                        let error_blob = result.0.get_error_buffer()?;
                        return Err(HassleError::ValidationError(
                            self.library.get_blob_as_string(&error_blob.into())?,
                        ));
                    }
                };

                Ok(result_blob.to_vec())
            }
        }
    }
}
