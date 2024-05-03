use pyo3::types::{PyAnyMethods, PyModule};
use pyo3::{Py, PyAny, PyResult, Python};

pub fn to_excel(input: String, output: String) -> PyResult<()> {
    println!("Start to convert to excel {} to {}", input, output);
    Python::with_gil(|py| {
        println!(
            "Start to convert to excel {} to {} with python.",
            input, output
        );
        let bound = PyModule::from_code_bound(
            py,
            r#"
def to_excel(input_path: str, output_path: str):
    import pandas as pd
    pd.read_csv(input_path, encoding='utf-8').to_excel(output_path, index=False)
"#,
            "",
            "",
        );
        if let Err(_) = bound {
            return Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Failed to get Python function",
            ));
        }
        let bound: Py<PyAny> = bound.unwrap().getattr("to_excel")?.into();
        let _ = bound.call1(py, (input.as_str(), output.as_str()));
        println!("Converted to excel {} to {}", input, output);
        Ok(())
    })
}
