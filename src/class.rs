use crate::binds::MonoClass;
use crate::Image;
use crate::Method;

use std::ffi::CString;
use core::ffi::c_void;

#[derive(Eq)]
pub struct Class{
    class_ptr:*mut MonoClass,
} 
impl Class{
    ///Returns copy of internal pointer to [`MonoClass`].
    pub fn get_ptr(&self)->*mut MonoClass{
        return self.class_ptr;
    }
    /// Creates [`Class`] from *class_ptr*. If it is not null, returns [`Some`], otherwise [`None`].
    /// # Safety
    /// *class_ptr* must me either a valid pointer to [`MonoClass`] or null pointer.
    pub unsafe fn from_ptr(class_ptr:*mut MonoClass)->Option<Self>{
        if class_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{class_ptr:class_ptr});
    }
    ///Returns class named *name* in *namespace* in image *image*.
    /// # Example
    ///```rust
    /// let some_class = Class::from_name(&some_image,"SomeNamespace","SomeClass").expect("Could not find a class!");
    ///```
    pub fn from_name(image:&crate::image::Image,namespace:&str,name:&str)->Option<Self>{
        let cstr_nspace = CString::new(namespace).expect("Could not create CString");
        let cstr_name = CString::new(name).expect("Could not create CString");
        let res = unsafe{crate::binds::mono_class_from_name(image.to_ptr(),cstr_nspace.as_ptr(),cstr_name.as_ptr())};
        return unsafe{Self::from_ptr(res)};
    } 
    ///Case sensitve version of Class::from_name.
    pub fn from_name_case(image:&crate::image::Image,namespace:&str,name:&str)->Option<Self>{
        let cstr_nspace = CString::new(namespace).expect("Could not create CString");
        let cstr_name = CString::new(name).expect("Could not create CString");
        let res = unsafe{crate::binds::mono_class_from_name_case(image.to_ptr(),cstr_nspace.as_ptr(),cstr_name.as_ptr())};
        return unsafe{Self::from_ptr(res)};
    } 
    ///Gets field *name* of class.
    /// # Example
    /// ## C#
    ///```csharp
    /// class SomeClass{
    ///     int someField;    
    /// }
    ///```
    /// ## Rust
    ///```rust
    /// let some_field = some_class.get_field_from_name("someField").expect("Could not find field!");
    ///```
    pub fn get_field_from_name(&self,name:&str)->Option<ClassField>{
        let cstr = CString::new(name).expect("Could not create CString");
        let res = unsafe{ClassField::from_ptr(crate::binds::mono_class_get_field_from_name(self.get_ptr(),cstr.as_ptr()))};
        drop(cstr);
        return res;
    }
    ///Returns name of this class
    pub fn get_name(&self)->String{
        let cstr = unsafe{CString::from_raw(crate::binds::mono_class_get_name(self.class_ptr) as *mut i8)};
        let res = cstr.to_str().expect("Could not covert CString to String!").to_owned();
        //pointer does not have to be released
        let _ = unsafe{cstr.into_raw()};
        return res;
    }
    ///Gets the image this type exists in.
    pub fn get_image(&self)->Image{
        return unsafe{Image::from_ptr(crate::binds:: mono_class_get_image(self.class_ptr))};
    }
    ///Returns ammount of memory occupied by object when inside array.
    pub fn array_element_size(&self)->i32{
        return unsafe{crate::binds:: mono_class_array_element_size(self.class_ptr)};
    }
    ///Gets a [`Vec`] contining interafces this class implements.
    pub fn get_interfaces(&self)->Vec<Class>{
        let mut gptr = 0 as *mut i32;
        let mut res = Vec::new();
        while let Some(class) = unsafe{Self::from_ptr(
            crate::binds::mono_class_get_interfaces(self.class_ptr,&mut gptr as *mut *mut  i32 as *mut *mut std::os::raw::c_void)
        )}{
            res.push(class);
        }
        return res;
    }
    ///Gets namespace this class is in, or "" string if it is not in any namespace.
    pub fn get_namespace(&self)->String{
        let cstr = unsafe{CString::from_raw(crate::binds::mono_class_get_namespace(self.class_ptr) as *mut i8)};
        let res = cstr.to_str().expect("Could not create CString!").to_owned();
        //got const pointer that does not have to be released.
        let _ = cstr.into_raw();
        return res;
    }
    ///Gets class this class is nested in, or None if it is not nested in any type.
    pub fn get_nesting_type(&self)->Option<Class>{
        return unsafe{Self::from_ptr(
            crate::binds::mono_class_get_nesting_type(self.class_ptr)
        )};
    }
    ///Gets type this class derives from or None if it does not derive any type.
    /// # Example
    /// For a class `SomeClass`
    /// # C#
    ///```csharp
    /// class SomeClass:SomeParrentClass{
    ///    
    /// }
    ///```
    ///
    /// Function will return `SomeParrentClass`
    pub fn get_parent(&self)->Option<Class>{
        return unsafe{Self::from_ptr(
            crate::binds::mono_class_get_parent(self.class_ptr)
        )};
    }
    ///Gets number of dimmension of array.
    /// # Constrins 
    /// *self* must be an array type.
    pub fn get_rank(&self)->i32{
        return unsafe{crate::binds::mono_class_get_rank(self.class_ptr)};
    }
    ///Return size of static data of this class
    pub fn data_size(&self)->i32{
        return unsafe{crate::binds::mono_class_data_size(self.class_ptr)};
    }
    ///Get element class of an array. *self* **must** be an array type.
    pub fn get_element_class(&self)->Class{
        return unsafe{Self::from_ptr(
            crate::binds::mono_class_get_element_class(self.class_ptr))
        }.expect("Colud not get array element class!");
    }
    //Returns wenether if class implements interface `iface`.
    pub fn implements_interface(&self,iface:&Self)->bool{
        return unsafe{crate::binds::mono_class_implements_interface(self.class_ptr,iface.class_ptr)} != 0;
    } 
    /// Returns true if object of type *other* can be assigned to class *self*.
    pub fn is_assignable_from(&self,other:&Self)->bool{
        return unsafe{crate::binds::mono_class_is_assignable_from(self.class_ptr,self.class_ptr)} != 0;
    }
    ///Checks if *self* represents a delegate type.
    pub fn is_delegate(&self)->bool{
        return unsafe{crate::binds::mono_class_is_delegate(self.class_ptr)} != 0;
    }
    ///Checks if *self* represents an enumeration type.
    pub fn is_enum(&self)->bool{
        return unsafe{crate::binds::mono_class_is_enum(self.class_ptr)} != 0;
    }
    //TODO: consider implementing mono_class_is_subclass_of(it seems mostly redundant, but it may be useful)
    //TODO: figure out what exacly mono_class_num_events is supposed to do, and implement it.
    ///Gets ammount of **static and instace** files of class
    pub fn num_fields(&self)->i32{
        return unsafe{crate::binds::mono_class_num_fields(self.class_ptr)};
    }
    ///Gets ammount of methods in the class *self*
    pub fn num_methods(&self)->i32{
        return unsafe{crate::binds::mono_class_num_methods(self.class_ptr)};
    }
    //TODO: expand this description, sice it does not seam to be fully clear.
    ///Gets number of properties in the class
    pub fn num_properties(&self)->i32{
        return unsafe{crate::binds::mono_class_num_properties(self.class_ptr)};
    }
    ///Checks if *self* represents a value type.
    pub fn is_valuetype(&self)->bool{
        return unsafe{crate::binds::mono_class_is_valuetype(self.class_ptr)} != 0;
    }
    /*
    TODO:figure out how this function works and fix it.
    ///Gets size of a value of type *self*
    pub fn value_size(&self)->i32{
        return unsafe{crate::binds::mono_class_value_size(self.class_ptr)};
    }
    */
    ///Returns [`Class`] representing `System.Object` type.
    pub fn get_object()->Class{
        return unsafe{Self::from_ptr(
            crate::binds::mono_get_object_class()
        )}.expect("Could not get calls representing System.Object!");
    }
    ///Returns [`Class`] representing `System.Int16` type ([i16]).
    pub fn get_int_16()->Class{
        return unsafe{Self::from_ptr(
            crate::binds::mono_get_int16_class()
        )}.expect("Could not get calls representing System.Int16!");
    }
    ///Returns [`Class`] representing `System.Int32` type ([i32]). 
    pub fn get_int_32()->Class{
        return unsafe{Self::from_ptr(
            crate::binds::mono_get_int32_class()
        )}.expect("Could not get calls representing System.Int32!");
    }
    ///Returns [`Class`] representing `System.Int64` type ([i64]). 
    pub fn get_int_64()->Class{
        return unsafe{Self::from_ptr(
            crate::binds::mono_get_int64_class()
        )}.expect("Could not get calls representing System.Int64!");
    }
    ///Returns [`Class`] representing `System.Double` type ([f64]).
    pub fn get_double()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_double_class()
        )}.expect("Could not get calls representing System.Double!");
    }
    ///Returns [`Class`] representing `System.Enum` type. 
    pub fn get_enum()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_enum_class()
        )}.expect("Could not get calls representing System.Enum!");
    }
    ///Returns [`Class`] representing `System.IntPtr` type ([isize]). 
    pub fn get_int_ptr()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_intptr_class()
        )}.expect("Could not get calls representing System.IntPtr!");
    }
    ///Returns [`Class`] representing `System.SByte` type ([i8]). 
    pub fn get_sbyte()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_sbyte_class()
        )}.expect("Could not get calls representing System.IntPtr!");
    }
    ///Returns [`Class`] representing `System.Single` type ([f32]).
    pub fn get_single()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_single_class()
        )}.expect("Could not get calls representing System.Single!");
    }
    ///Returns [`Class`] representing `System.String` type.
    pub fn get_string()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_string_class()
        )}.expect("Could not get calls representing System.String!");
    }
    ///Returns [`Class`] representing `System.Threading.Thread` type.
    pub fn get_thread()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_thread_class()
        )}.expect("Could not get calls representing System.Threading.Thread!");
    }
    ///Returns [`Class`] representing `System.UInt16` type([u16]).
    pub fn get_uint16()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_uint16_class()
        )}.expect("Could not get calls representing System.UInt16!");
    }
    ///Returns [`Class`] representing `System.UInt32` type([u32]).
    pub fn get_uint32()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_uint32_class()
        )}.expect("Could not get calls representing System.UInt32!");
    }
    ///Returns [`Class`] representing `System.UInt64` type([u64]).
    pub fn get_uint64()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_uint64_class()
        )}.expect("Could not get calls representing System.UInt64!");
    }
    ///Returns [`Class`] representing `System.UIntPtr` type ([usize]). 
    pub fn get_uint_ptr()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_uintptr_class()
        )}.expect("Could not get calls representing System.IntPtr!");
    }
    ///Returns [`Class`] representing `System.Void` type. 
    pub fn get_void()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_void_class()
        )}.expect("Could not get calls representing System.Void!");
    }
    ///Returns [`Class`] representing `System.Array` type. 
    pub fn get_array()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_array_class()
        )}.expect("Could not get calls representing System.Array!");
    }
    ///Returns [`Class`] representing `System.Boolean` type ([bool]). 
    pub fn get_boolean()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_boolean_class()
        )}.expect("Could not get calls representing System.Boolean!");
    }
    ///Returns [`Class`] representing `System.Byte` type ([u8]). 
    pub fn get_byte()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_byte_class()
        )}.expect("Could not get calls representing System.Byte!");
    }
    ///Returns [`Class`] representing `System.Char` type ([char]). 
    pub fn get_char()->Class{
        return unsafe{Self::from_ptr(
            crate::binds:: mono_get_char_class()
        )}.expect("Could not get calls representing System.Char!");
    }
    ///Returns all fields of a class
    pub fn get_fields(&self)->Vec<ClassField>{
        let mut gptr = 0 as *mut std::os::raw::c_void;
        let mut res = Vec::new();
        while let Some(cf) = unsafe{ClassField::from_ptr(
            crate::binds::mono_class_get_fields(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            res.push(cf);
        }
        return res;
    }
    ///Returns all methods of a class
    pub fn get_methods(&self)->Vec<Method>{
        let mut gptr = 0 as *mut std::os::raw::c_void;
        let mut res = Vec::new();
        while let Some(cf) = unsafe{Method::from_ptr(
            crate::binds::mono_class_get_methods(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            res.push(cf);
        }
        return res;
    }
    //Gets all types nested inside this class.
    ///Returns all methods of a class
    pub fn get_nested_types(&self)->Vec<Class>{
        let mut gptr = 0 as *mut std::os::raw::c_void;
        let mut res = Vec::new();
        while let Some(cf) = unsafe{Class::from_ptr(
            crate::binds::mono_class_get_nested_types(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            res.push(cf);
        }
        return res;
    }
    //TODO:mono_class_get_property_from_name && mono_class_get_properties ASAP
}
impl std::cmp::PartialEq for Class{
    fn eq(&self,other:&Self)->bool{
        return self.class_ptr == other.class_ptr;
    }
}
use crate::object::Object;
use crate::binds::MonoClassField;
pub struct ClassField{
    cf_ptr:*mut MonoClassField,
}

impl ClassField{
    /// Creates [`ClassField`] form *cf_ptr*. Returns [`Some(ClassField)`] if pointer is not null, and [`None`] if it is.
    /// # Safety
    /// *cf_ptr* must be either a valid pointer to [`MonoClassField`] or null pointer.
    pub fn from_ptr(cf_ptr:*mut MonoClassField)->Option<Self>{
        if cf_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{cf_ptr:cf_ptr});
    }
    /// Gets internal [`MonoClassField`] pointer.
    pub fn get_ptr(&self)->*mut MonoClassField{
        return self.cf_ptr;
    }
    ///Gets the name of [`ClassField`]
    /// # Example
    ///```rust
    /// let some_field_name = "someField".
    /// let some_field = some_class.get_field_from_name(some_field_name).expect("Could not find field!");
    /// let name = some_field.get_name();
    /// assert!(some_filed_name == name);
    ///```
    pub fn get_name(&self)->String{
        let cstr = unsafe{std::ffi::CString::from_raw(crate::binds::mono_field_get_name(self.get_ptr()) as *mut i8)};
        let s = cstr.to_str().expect("Could not create String from ptr").to_owned();
        drop(cstr);
        return s;
    }
    ///get metadata(???) of a field. **not** it's value
    pub fn get_data(&self) -> *const ::std::os::raw::c_char{
        return unsafe{crate::binds::mono_field_get_data(self.get_ptr())};
    }
    ///Returns [`Class`] this field is attached to.
    /// # Example
    ///```rust
    /// let some_field = some_class.get_field_from_name(some_field_name).expect("Could not find field!");
    /// let some_field_class = some_field.get_parrent();
    /// assert!(some_field_class == some_class);
    ///```
    pub fn get_parent(&self)->Class{
        return unsafe{Class::from_ptr(crate::binds:: mono_field_get_parent(self.get_ptr()))}.expect("Could not get ClassFiled of Class");
    }
    ///Gets value of a field on [`Object`] *obj*. For boxable types this value is in boxed form. 
    ///In this case call [`Object`].unbox() to retrive pointer to unboxed version of this value.
    /// # Example
    /// ## C#
    ///```csharp
    /// class SomeClass{
    ///     int someField;    
    /// }
    ///```
    /// ## Rust
    ///```rust
    /// let some_field_value_object = some_field.get_value_object(&instance_of_some_class);
    /// //Retrived value *some_field_value_object* is a boxed int. 
    /// let some_field_value = *(some_field_value_object.unbox() as *mut i32);
    /// //First got pointer to unboxed value using unbox() then converted it to proper type (*mut i32), and dereferenced
    /// it to get its value.
    /// ```
    pub fn get_value_object(&self,obj:&Object)->Option<Object>{
        use crate::object::ObjectTrait;
        let dom = obj.get_domain();
        return unsafe{Object::from_ptr(
            crate::binds::mono_field_get_value_object(dom.get_ptr(),self.get_ptr(),obj.get_ptr())
        )};
    }
    ///Sets value of the object field on [`Object`] to value pointed to by *value*
    /// # Example
    /// ## C#
    ///```csharp
    /// class SomeClass{
    ///     int someField;    
    /// }
    ///```
    /// ## Rust
    ///```rust
    /// let value_to_set:i32 = 11;
    /// let some_field_value_object = some_field.set_value_unsafe(&instance_of_some_class,&mut value_to_set as *mut i32 as *mut  std::os::raw::c_void);
    /// ```
    /// # Safety
    /// *value_ptr* pointer must be valid and have correct type.
    pub unsafe fn set_value_unsafe(&self,obj:&crate::object::Object,value_ptr:*mut std::os::raw::c_void){
        crate::binds::mono_field_set_value(obj.get_ptr(),self.get_ptr(),value_ptr);
    }
}