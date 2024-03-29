use crate::nric::NRIC;
use crate::secret::SecretNRICString;
use pyo3::exceptions::PyValueError;
use pyo3::{prelude::*, types::PyString};

#[pyclass(name = "SecretNRIC")]
#[pyo3(text_signature = "(string, filepath, key_var)")]
#[derive(Debug)]
pub struct PySecretNRIC {
    pub rust_secret_nric: SecretNRICString,
}

#[pymethods]
impl PySecretNRIC {
    #[new]
    pub fn new(string: &PyString, filepath: &str, key_var: &str) -> PyResult<Self> {
        Python::with_gil(move |_py| {
            let string = string.extract::<String>()?;
            let new_nric = NRIC::new(string);
            match new_nric {
                Ok(new_nric) => Ok(PySecretNRIC {
                    rust_secret_nric: SecretNRICString::new(new_nric, filepath, key_var)?,
                }),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        })
    }

    pub fn __str__(&self) -> PyResult<&str> {
        Ok("<SECRETNRIC>")
    }

    pub fn __repr__(&self) -> PyResult<&str> {
        Ok("<SECRETNRIC>")
    }

    pub fn reveal_encrypted(&self) -> PyResult<&str> {
        Ok(&self.rust_secret_nric.encrypted_nric)
    }

    #[staticmethod]
    pub fn decrypt(input: &str, filepath: &str, key: &str) -> anyhow::Result<String> {
        SecretNRICString::decrypt(input, filepath, key)
    }
}
