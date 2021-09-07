use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use monte_carlo_pi::monte_carlo_pi;
/// PyResult<(f64, String) are the return arguments 
#[pyfunction] // This makes it a python function 
fn mcpi(iterations: u32) -> PyResult<(f64, String)> {
    Ok(monte_carlo_pi(iterations as u32))
}

#[pymodule]
fn montecarlopi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mcpi, m)?)?;

    Ok(())
}

// cargo test [testname]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}