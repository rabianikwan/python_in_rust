use anyhow::Ok;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn main() -> Result<(), anyhow::Error>{
    let arg1 = "arg1";
    let arg2 = "arg2";

    let result= Python::with_gil(|py| -> PyResult<PyObject> {
        let fun: PyObject = PyModule::from_code(
            py,
            "
import numpy
def example():
    a = numpy.array([1, 2, 3])
    return a",
            "",
            "",
        )?
        .getattr("example")?
        .into();
        //  args is argument for example if example have an argument
        let args = PyTuple::new(py, &[arg1, arg2]);
        let result = fun.call0(py);
        result
    })?;
    println!("result: {:?}", result);

    Ok(())

}
