// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod clustering {
    pub mod rs {
      
      #[allow(clippy::all)]
      pub mod cluster {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "clustering:rs/cluster#run"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_run(arg0: i32,arg1: i32,arg2: i32,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let base3 = arg0;
            let len3 = arg1;
            let mut result3 = Vec::with_capacity(len3 as usize);
            for i in 0..len3 {
              let base = base3 + i * 8;
              let e3 = {
                let l0 = *((base + 0) as *const i32);
                let l1 = *((base + 4) as *const i32);
                let len2 = l1 as usize;
                
                Vec::from_raw_parts(l0 as *mut _, len2, len2)
              };
              result3.push(e3);
            }
            wit_bindgen::rt::dealloc(base3, (len3 as usize) * 8, 4);
            let result4 = <_GuestImpl as Guest>::run(result3, arg2 as u32);
            let ptr5 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec6 = (result4).into_boxed_slice();
            let ptr6 = vec6.as_ptr() as i32;
            let len6 = vec6.len() as i32;
            ::core::mem::forget(vec6);
            *((ptr5 + 4) as *mut i32) = len6;
            *((ptr5 + 0) as *mut i32) = ptr6;
            ptr5
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_clustering:rs/cluster#run"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_run(arg0: i32,) {
              let l0 = *((arg0 + 0) as *const i32);
              let l1 = *((arg0 + 4) as *const i32);
              let base2 = l0;
              let len2 = l1;
              wit_bindgen::rt::dealloc(base2, (len2 as usize) * 4, 4);
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn run(points: wit_bindgen::rt::vec::Vec::<wit_bindgen::rt::vec::Vec::<f64>>,n_clusters: u32,) -> wit_bindgen::rt::vec::Vec::<u32>;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(4))]
        struct _RetArea([u8; 8]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 8]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:clustering"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 328] = [3, 0, 10, 99, 108, 117, 115, 116, 101, 114, 105, 110, 103, 0, 97, 115, 109, 13, 0, 1, 0, 7, 74, 1, 65, 2, 1, 66, 5, 1, 112, 117, 1, 112, 0, 1, 112, 121, 1, 64, 2, 6, 112, 111, 105, 110, 116, 115, 1, 10, 110, 45, 99, 108, 117, 115, 116, 101, 114, 115, 121, 0, 2, 4, 0, 3, 114, 117, 110, 1, 3, 4, 1, 21, 99, 108, 117, 115, 116, 101, 114, 105, 110, 103, 58, 114, 115, 47, 99, 108, 117, 115, 116, 101, 114, 5, 0, 11, 13, 1, 0, 7, 99, 108, 117, 115, 116, 101, 114, 3, 0, 0, 7, 106, 1, 65, 2, 1, 65, 2, 1, 66, 5, 1, 112, 117, 1, 112, 0, 1, 112, 121, 1, 64, 2, 6, 112, 111, 105, 110, 116, 115, 1, 10, 110, 45, 99, 108, 117, 115, 116, 101, 114, 115, 121, 0, 2, 4, 0, 3, 114, 117, 110, 1, 3, 4, 1, 21, 99, 108, 117, 115, 116, 101, 114, 105, 110, 103, 58, 114, 115, 47, 99, 108, 117, 115, 116, 101, 114, 5, 0, 4, 1, 24, 99, 108, 117, 115, 116, 101, 114, 105, 110, 103, 58, 114, 115, 47, 99, 108, 117, 115, 116, 101, 114, 105, 110, 103, 4, 0, 11, 16, 1, 0, 10, 99, 108, 117, 115, 116, 101, 114, 105, 110, 103, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
