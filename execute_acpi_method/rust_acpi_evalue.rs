// SPDX-License-Identifier: GPL-2.0

//! Rust hello_world
//executa um metodo acpi

use kernel::prelude::*;
use core::ffi::c_void;

extern "C" {
    //#[allow(dead_code)]
    fn acpi_get_handle(
        parent: *const c_void,
        pathname: *const u8,
        ret_handle: *mut *const c_void,
    ) -> i32;
     
    #[allow(dead_code)]
    fn acpi_evaluate_object(
        handle: *const c_void,
        pathname: *const u8,
        external_params: *mut c_void,
        result: *mut c_void,
    ) -> i32;
}

fn execute_acpi_method() -> Result<(), i32> {
     let mut handle: *const c_void = core::ptr::null();
     let method_name = b"\\_SB.PCI0.SBRG.H_EC._Q54\0"; // Nome do método ACPI a ser executado
                                                      
unsafe {
     let ret = acpi_get_handle(core::ptr::null(), method_name.as_ptr(), &mut handle);
      if ret != 0 {
            pr_err!("Failed to get ACPI handle: {}\n", ret);
            return Err(ret);
        } 

// Executa o método ACPI
        let result: *mut c_void = core::ptr::null_mut();
        let ret = acpi_evaluate_object(handle, method_name.as_ptr(), core::ptr::null_mut(), result);
        if ret != 0 {
            pr_err!("Failed to evaluate ACPI method: {}\n", ret);
            return Err(ret);
        }
}
Ok(())
}
module! {
    type: RustAcpi,
    name: "rust_hello",
    author: "Edson Juliano Drosdeck",
    description: "Rust hello world",
    license: "GPL",
}

struct RustAcpi;

impl kernel::Module for RustAcpi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World RUST\n");
       let _ = execute_acpi_method();
        Ok(Self)
    }
}

impl Drop for RustAcpi {
    fn drop(&mut self) {
        pr_info!("Rust Hello world (exit)\n");
    }
}



