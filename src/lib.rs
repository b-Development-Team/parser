pub mod parser;
use pyo3::prelude::*;
use crate::parser::{parse, Property, Type};
use crate::parser::Type::PROGRAM;

#[pyfunction]
fn parse_wrapped(str: &str) -> PyResult<Property> {
    let code = str.to_string();
    Ok(parse(&mut code.clone(), PROGRAM, 0, 1,&mut vec![]))
}

#[pymodule]
fn bstarparser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_wrapped, m)?)?;
    m.add_class::<Property>()?;
    m.add_class::<Type>()?;

    Ok(())
}