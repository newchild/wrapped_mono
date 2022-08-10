use crate::binds::{mono_jit_init,mono_jit_init_version,mono_config_parse,mono_jit_cleanup,mono_jit_exec};
use crate::domain::{Domain};
use std::ffi::CString;
use core::ptr::null_mut;
/// This function starts up MonoRuntime,and returns main domain. It should be called before any other mono function is called. **Can be only called once per process.**
/// Version argument specifies runtime version, if **None** passed, default version will be selected.
/// ```rust
/// let main_domain = jit::init("domain_name",None);
/// ```
/// ```rust
/// let main_domain_with_version = jit::init("domain_name","v4.0.30319");
/// ```
pub fn init(name:&str,version:Option<&str>)->Domain{
    let n_cstr = CString::new(name).expect("could not create cstring!");
    let res = unsafe{Domain::create_from_ptr( match version{
        Some(s)=>{
            let v_cstr = CString::new(s).expect("could not create cstring!");
            let ptr = mono_jit_init_version(n_cstr.as_ptr(),v_cstr.as_ptr());
            drop(v_cstr);
            ptr
        },
        None=>{
            mono_jit_init(n_cstr.as_ptr())
        }
    })};
    unsafe{mono_config_parse (null_mut())};
    drop(n_cstr);
    return res;
}
/// This function shuts down MonoRuntime.
/// **WARNING!** after it is called, MonoRuntime **will not be** able to be used again in the same process, since it can be only started up once.
/// ```rust
/// let main_domain = jit::init("main",None);
/// // All code using MonoRuntime goes here
/// jit::cleanup(main_domain);
/// ```
pub fn cleanup(domain:Domain){
    unsafe{mono_jit_cleanup(domain.get_ptr())};
}
use crate::assembly::{Assembly,AssemblyTrait};
/// Function used to call main function from assembly in domain with arguments.
/// ```csharp
/// //C# code in file "SomeAssembly.dll"
/// class Apllication{
/// public static void Main(string args[]){
///     /*Some C# code*/   
///     }
/// }
/// ```
/// ```rust
/// let main_doamin = jit::init("main",None);
/// let asm = main_domain.assembly_open("SomeAssembly.dll");
/// let args = vec!["arg1","arg2","arg3"];
/// let res = jit::exec(main_domain,asm,args);
/// ```
pub fn exec(domain:Domain,assembly:Assembly,args:Vec<&str>)->i32{
    let argc:i32 = args.len() as i32;
    let mut cstr_args:Vec<CString> = Vec::new();
    let mut argv:Vec<*mut i8> = Vec::new();
    for arg in args{
        let cstr_arg = CString::new(arg).unwrap();
        argv.push(cstr_arg.as_ptr() as *mut i8);  
        cstr_args.push(cstr_arg); 
    }
    let res = unsafe{mono_jit_exec(domain.get_ptr(),assembly.get_ptr(),argc,argv.as_mut_ptr())};
    drop(cstr_args);
    return res;
}