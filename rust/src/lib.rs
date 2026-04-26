use pyo3::prelude::*;


#[pyfunction]
fn process(data: Vec<i32>) -> Vec<i32> {
    data.into_iter().map(|x| x * 2).collect()
}

#[pymodule]
mod rpipeline {
    #[pymodule_export]
    use crate::process;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(Vec::from([2,3])); // process([2,3].to_vec()); process([2,3].into());
        assert_eq!(result, [4, 6]);
    }
}
