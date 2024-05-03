use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub fn to_excel() -> Result<(), String> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let code = r#"
import pandas as pd

def read_and_save_csv(input_path, output_path):
    df = pd.read_csv(input_path, encoding='utf-8')
    df.to_excel(output_path, index=False)
"#;

    py.run(code, None, None).unwrap();

    let pandas = py.import("pandas");
    if let Err(_) = pandas {
        return Err("Failed to import pandas".to_string());
    }
    let pandas = pandas.unwrap();
    let locales = [("pd", pandas)].into_py_dict(py);
    let func = py.eval("read_and_save_csv", None, Some(locales));
    if let Err(_) = func {
        return Err("Failed to get Python function".to_string());
    }
    let func = func.unwrap().extract();
    if let Err(_) = func {
        return Err("Failed to extract Python function".to_string());
    }
    let func: Py<PyAny> = func.unwrap();
    let _ = func.call1(py, ("output.csv", "output.xlsx"));
    Ok(())
}
