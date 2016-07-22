#[macro_use] extern crate cpython;

use cpython::{ObjectProtocol, NoArgs, Python};

py_exception!(mymodule, CustomError);

struct Foo;

macro_rules! newtype(
    ($name: ident) => (
    //    py_impl_to_py_object_for_python_object!($name);
    //    py_impl_from_py_object_for_python_object!($name);
    #[derive(Debug)]
    pub struct $name;
    );
    ($name: ident, $checkfunction: ident) => (
    //    pyobject_newtype!($name);
    #[derive(Debug)]
    pub struct $name;
    );
    ($name: ident, $checkfunction: ident, $typeobject: ident) => (
    //    pyobject_newtype!($name, $checkfunction);
    #[derive(Debug)]
    pub struct $name;
    );
);

macro_rules! exception {
    ($module: ident, $name: ident, $base: ty) => {
        newtype!($name);
    };
    ($module: ident, $name: ident) => {
        exception!($module, $name, Foo);
    }
}

// exception!(mymodule, Bar);
// exception!(mymodule, Bax);
//
// #[test]
// fn test_banancitrom() {
//     let gil = Python::acquire_gil();
//     let py = gil.python();
//
//     let s = "citrom";
//
//     // let e = CustomError::new(py, NoArgs);
//     // let s_ = e.ptype.str(py).unwrap();
//     // let s = s_.to_string(py).unwrap();
//
//     println!("{}", s);
//     assert!(s == "alma");
//
//     assert_eq!(format!("{:?}", Bar), "Bar".to_string());
// }

#[test]
fn test_str() {
    assert!("banan" != "bitrom");

    let gil = Python::acquire_gil();
    let py = gil.python();

    let e = CustomError::new(py, NoArgs);
    let s_ = e.ptype.str(py).unwrap();
    let s = s_.to_string(py).unwrap();

    println!("{}", s);
    assert!(s == "alma");
}
