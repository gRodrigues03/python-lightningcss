use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::{Browsers, Targets};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Parses, minifies and prints CSS. Output is always minified.
/// `browsers_list` (browserslist queries) controls prefixing/transpilation.
#[pyfunction]
#[pyo3(signature = (code, /, filename="", browsers_list=None))]
fn process_stylesheet(
    code: &str,
    filename: &str,
    browsers_list: Option<Vec<String>>,
) -> PyResult<String> {
    let targets = match browsers_list {
        Some(list) if !list.is_empty() => Browsers::from_browserslist(list)
            .map(Targets::from)
            .map_err(|e| PyValueError::new_err(format!("Invalid browsers_list: {e}")))?,
        _ => Targets::default(),
    };

    let mut stylesheet = StyleSheet::parse(
        code,
        ParserOptions {
            filename: filename.to_string(),
            ..Default::default()
        },
    )
    .map_err(|e| PyValueError::new_err(format!("Parsing failed: {e}")))?;

    stylesheet
        .minify(MinifyOptions {
            targets,
            ..Default::default()
        })
        .map_err(|e| PyValueError::new_err(format!("Minifying failed: {e}")))?;

    let result = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            targets,
            ..Default::default()
        })
        .map_err(|e| PyValueError::new_err(format!("Printing failed: {e}")))?;

    Ok(result.code)
}

#[pymodule]
#[pyo3(name = "lightningcss")]
fn pylightningcss(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_stylesheet, m)?)
}