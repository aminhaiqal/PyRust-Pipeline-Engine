use pyo3::prelude::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[pyfunction]
fn py_add(left: u64, right: u64) -> u64 {
    add(left, right)
}

#[pymodule]
mod calc {
    #[pymodule_export]
    use crate::py_add;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
