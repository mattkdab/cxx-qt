mod my_object {
    use cxx_qt_lib::PropertyChangeHandler;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {
            Number,
            String,
        }

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;

            #[rust_name = "number"]
            fn getNumber(self: &MyObject) -> i32;
            #[rust_name = "set_number"]
            fn setNumber(self: Pin<&mut MyObject>, value: i32);

            #[rust_name = "string"]
            fn getString(self: &MyObject) -> &QString;
            #[rust_name = "set_string"]
            fn setString(self: Pin<&mut MyObject>, value: &QString);

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseMyObjectCpp"]
            fn initialise_my_object_cpp(cpp: Pin<&mut MyObject>);

            #[cxx_name = "handlePropertyChange"]
            fn call_handle_property_change(
                self: &mut RustObj,
                cpp: Pin<&mut MyObject>,
                property: Property,
            );
        }
    }

    pub type CppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn call_handle_property_change(
            &mut self,
            cpp: std::pin::Pin<&mut CppObj>,
            property: Property,
        ) {
            self.handle_property_change(cpp, property);
        }
    }

    pub struct CppObjWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> CppObjWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }

        pub fn number(&self) -> i32 {
            self.cpp.number()
        }

        pub fn set_number(&mut self, value: i32) {
            self.cpp.as_mut().set_number(value);
        }

        pub fn string(&self) -> &cxx_qt_lib::QString {
            self.cpp.string()
        }

        pub fn set_string(&mut self, value: &cxx_qt_lib::QString) {
            self.cpp.as_mut().set_string(value);
        }

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const CppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;

            data.number
                .map_qt_value(|context, converted| context.set_number(converted), self);
            data.string
                .map_qt_value(|context, converted| context.set_string(converted), self);
        }
    }

    #[derive(Default)]
    struct Data {
        number: i32,
        string: String,
    }

    impl<'a> From<&CppObjWrapper<'a>> for Data {
        fn from(value: &CppObjWrapper<'a>) -> Self {
            Self {
                number: value.number().into(),
                string: value.string().into(),
            }
        }
    }

    impl PropertyChangeHandler<CppObj, Property> for RustObj {
        fn handle_property_change(
            &mut self,
            _cpp: std::pin::Pin<&mut CppObj>,
            _property: Property,
        ) {
            println!("change")
        }
    }

    fn create_my_object_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    fn initialise_my_object_cpp(cpp: std::pin::Pin<&mut CppObj>) {
        let mut wrapper = CppObjWrapper::new(cpp);
        wrapper.grab_values_from_data(&Data::default());
    }
}
