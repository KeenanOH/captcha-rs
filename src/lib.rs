use captcha_rs::{CaptchaBuilder};

use pyo3::prelude::*;


#[pyclass(name="Captcha")]
struct PyCaptcha {
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub base_img: String,
    #[pyo3(get)]
    pub dark_mode: bool,
}

#[pyclass(name="CaptchaBuilder")]
struct PyCaptchaBuilder {
    pub length: Option<usize>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub dark_mode: Option<bool>,
    pub complexity: Option<u32>,
}

#[pymethods]
impl PyCaptchaBuilder {
    #[new]
    pub fn new() -> Self {
        Self {
            length: None,
            width: None,
            height: None,
            dark_mode: None,
            complexity: None
        }
    }

    pub fn length(mut slf: PyRefMut<Self>, length: usize) -> PyRefMut<Self> {
        slf.length = Some(length);
        slf
    }

    pub fn width(mut slf: PyRefMut<Self>, width: u32) -> PyRefMut<Self> {
        slf.width = Some(width);
        slf
    }

    pub fn height(mut slf: PyRefMut<Self>, height: u32) -> PyRefMut<Self> {
        slf.height = Some(height);
        slf
    }

    pub fn dark_mode(mut slf: PyRefMut<Self>, dark_mode: bool) -> PyRefMut<Self> {
        slf.dark_mode = Some(dark_mode);
        slf
    }

    pub fn complexity(mut slf: PyRefMut<Self>, complexity: u32) -> PyRefMut<Self> {
        let mut complexity = complexity;
        if complexity > 10 { complexity = 10; }
        if complexity < 1 { complexity = 1; }
        slf.complexity = Some(complexity);
        slf
    }

    pub fn build(&self) -> PyCaptcha {
        let builder = CaptchaBuilder::new()
            .length(self.length.unwrap())
            .width(self.width.unwrap())
            .height(self.height.unwrap())
            .dark_mode(self.dark_mode.unwrap())
            .complexity(self.complexity.unwrap());
        let captcha = builder.build();
        PyCaptcha {
            text: captcha.text,
            base_img: captcha.base_img,
            dark_mode: captcha.dark_mode
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn captcha_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyCaptcha>()?;
    m.add_class::<PyCaptchaBuilder>()?;
    Ok(())
}
