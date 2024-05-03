use pyo3::prelude::*;

fn main() {
    Python::with_gil(|py| {
        let r = PyModule::import_bound(py, "matplotlib.figure");
        println!("{r:?}");
    })
}
