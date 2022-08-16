use crate::binds::{MonoDomain, mono_domain_create,mono_domain_assembly_open};
use crate::assembly::{Assembly};
use core::ptr::null_mut;
/// Safe representation of MonoDoamin type.
#[derive(Eq)]
pub struct Domain{
    ptr:*mut MonoDomain,
} 
use std::ffi::CString;
impl Domain{ 
    ///Loads [`Assembly`] at path into domain, returns **None** if assembly could not be loaded(is missing or broken), and **Some(Assembly)** if it was succesfuly loaded. 
    pub fn assembly_open(&self,path:&str)->Option<Assembly>{
        //! # Example
        //!```rust
        //! let asm = domain.assembly_open("SomeAssembly.dll").expect("Could not load assembly!");
        //!```
        let cstr = CString::new(path).expect("Couldn't create cstring!");
        let ptr = unsafe{mono_domain_assembly_open(self.get_ptr(),cstr.as_ptr())};
        if ptr == null_mut(){
            return None;
        }
        drop(cstr);
        return Some(unsafe{Assembly::from_ptr(ptr)});
    }
    /// Creates new empty domain
    /// # Example
    /// ```rust
    /// let domain1 = jit::init();
    /// let domain2 = Domain::create();
    /// ```
    pub fn create()->Domain{
        
        return unsafe{Self::from_ptr(mono_domain_create())};
    }
    /// Sets domain confing to one loaded from file *filename* in directory *base_directory*.
    pub fn set_config(&self,base_directory:&str,filename:&str){
        let bd_cstr = CString::new(base_directory).expect("Could not create CString");
        let fnme_cstr =CString::new(filename).expect("Could not create CString");
        unsafe{crate::binds::mono_domain_set_config(self.ptr,bd_cstr.as_ptr(),fnme_cstr.as_ptr())};
        drop(bd_cstr);
        drop(fnme_cstr);
    }
    /// Function creating MonoDomain type from a pointer.
    /// #Safety
    /// Pointer must be a valid pointer to MonoDomain.
    pub unsafe fn from_ptr(ptr:*mut MonoDomain)->Domain{
        return Self{ptr:ptr};
    }
    /// Function returning internal pointer to MonoDomain
    pub unsafe fn get_ptr(&self)->*mut MonoDomain{
        return self.ptr;
    }
    /// Releases resources realted to a specific domain. If *force* is true, allows realesing of the root domain. Used during shutdown.
    /// # Safety
    /// Since this function releases all resurces realated to given domain, it means that all references to objects inside it will become invalid.
    /// 
    pub fn free(&self,force:bool){
        unsafe{crate::binds::mono_domain_free(self.ptr,force as i32)};
        drop(self);
    }
}
impl std::cmp::PartialEq for Domain{
    fn eq(&self, other: &Self) -> bool {
        return self.ptr == other.ptr;
    }
}