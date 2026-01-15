// Aevolang - 自进化编程语言库 / Self-evolving Programming Language Library
// Python模块导出 / Python module exports

mod evolution;
mod grammar;
mod parser;
mod poetry;
mod python;
mod runtime;

pub use evolution::*;
pub use grammar::*;
pub use parser::*;
pub use poetry::*;
pub use python::*;
pub use runtime::*;

// PyO3 Python模块导出 / PyO3 Python module exports
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

/// Python模块：Aevolang解析器和解释器
/// Python module: Aevolang parser and interpreter
#[pymodule]
fn aevo(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<AevoInterpreter>()?;
    m.add_class::<AevoParser>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    m.add_function(wrap_pyfunction!(eval, m)?)?;
    Ok(())
}

/// Aevolang解释器Python包装类
/// Aevolang interpreter Python wrapper class
#[pyclass]
pub struct AevoInterpreter {
    interpreter: runtime::Interpreter,
}

#[pymethods]
impl AevoInterpreter {
    /// 创建新解释器 / Create new interpreter
    #[new]
    fn new() -> Self {
        Self {
            interpreter: runtime::Interpreter::new(),
        }
    }

    /// 执行Aevolang代码 / Execute Aevolang code
    fn execute(&mut self, code: &str) -> PyResult<String> {
        let parser = parser::AdaptiveParser::new(true);
        match parser.parse(code) {
            Ok(ast) => {
                match self.interpreter.execute(&ast) {
                    Ok(value) => Ok(value.to_string()),
                    Err(e) => Err(PyValueError::new_err(format!("Execution error: {:?}", e))),
                }
            }
            Err(e) => Err(PyValueError::new_err(format!("Parse error: {:?}", e))),
        }
    }

    /// 执行代码并返回结果值 / Execute code and return result value
    fn eval(&mut self, code: &str) -> PyResult<PyObject> {
        let parser = parser::AdaptiveParser::new(true);
        match parser.parse(code) {
            Ok(ast) => {
                match self.interpreter.execute(&ast) {
                    Ok(value) => {
                        Python::with_gil(|py| {
                            Ok(value_to_pyobject(py, &value))
                        })
                    }
                    Err(e) => Err(PyValueError::new_err(format!("Execution error: {:?}", e))),
                }
            }
            Err(e) => Err(PyValueError::new_err(format!("Parse error: {:?}", e))),
        }
    }
}

/// Aevolang解析器Python包装类
/// Aevolang parser Python wrapper class
#[pyclass]
pub struct AevoParser {
    parser: parser::AdaptiveParser,
}

#[pymethods]
impl AevoParser {
    /// 创建新解析器 / Create new parser
    #[new]
    fn new(enable_nlu: bool) -> Self {
        Self {
            parser: parser::AdaptiveParser::new(enable_nlu),
        }
    }

    /// 解析Aevolang代码 / Parse Aevolang code
    fn parse(&self, code: &str) -> PyResult<PyObject> {
        match self.parser.parse(code) {
            Ok(ast) => {
                Python::with_gil(|py| {
                    Ok(ast_to_pyobject(py, &ast))
                })
            }
            Err(e) => Err(PyValueError::new_err(format!("Parse error: {:?}", e))),
        }
    }
}

/// 解析Aevolang代码并返回AST（Python字典格式）
/// Parse Aevolang code and return AST (as Python dict)
#[pyfunction]
fn parse(code: &str) -> PyResult<PyObject> {
    let parser = parser::AdaptiveParser::new(true);
    match parser.parse(code) {
        Ok(ast) => {
            Python::with_gil(|py| {
                Ok(ast_to_pyobject(py, &ast))
            })
        }
        Err(e) => Err(PyValueError::new_err(format!("Parse error: {:?}", e))),
    }
}

/// 执行Aevolang代码并返回结果字符串
/// Execute Aevolang code and return result string
#[pyfunction]
fn execute(code: &str) -> PyResult<String> {
    let parser = parser::AdaptiveParser::new(true);
    let mut interpreter = runtime::Interpreter::new();
    match parser.parse(code) {
        Ok(ast) => {
            match interpreter.execute(&ast) {
                Ok(value) => Ok(value.to_string()),
                Err(e) => Err(PyValueError::new_err(format!("Execution error: {:?}", e))),
            }
        }
        Err(e) => Err(PyValueError::new_err(format!("Parse error: {:?}", e))),
    }
}

/// 执行Aevolang代码并返回Python对象
/// Execute Aevolang code and return Python object
#[pyfunction]
fn eval(code: &str) -> PyResult<PyObject> {
    let parser = parser::AdaptiveParser::new(true);
    let mut interpreter = runtime::Interpreter::new();
    match parser.parse(code) {
        Ok(ast) => {
            match interpreter.execute(&ast) {
                Ok(value) => {
                    Python::with_gil(|py| {
                        Ok(value_to_pyobject(py, &value))
                    })
                }
                Err(e) => Err(PyValueError::new_err(format!("Execution error: {:?}", e))),
            }
        }
        Err(e) => Err(PyValueError::new_err(format!("Parse error: {:?}", e))),
    }
}

/// 将Aevolang Value转换为Python对象
/// Convert Aevolang Value to Python object
fn value_to_pyobject(py: Python, value: &runtime::interpreter::Value) -> PyObject {
    match value {
        runtime::interpreter::Value::Int(i) => i.to_object(py),
        runtime::interpreter::Value::Float(f) => f.to_object(py),
        runtime::interpreter::Value::String(s) => s.to_object(py),
        runtime::interpreter::Value::Bool(b) => b.to_object(py),
        runtime::interpreter::Value::Null => py.None(),
        runtime::interpreter::Value::List(list) => {
            let py_list = pyo3::types::PyList::empty_bound(py);
            for item in list {
                py_list.append(value_to_pyobject(py, item)).unwrap();
            }
            py_list.into()
        }
        runtime::interpreter::Value::Dict(dict) => {
            let py_dict = pyo3::types::PyDict::new_bound(py);
            for (key, val) in dict {
                py_dict.set_item(key, value_to_pyobject(py, val)).unwrap();
            }
            py_dict.into()
        }
    }
}

/// 将AST转换为Python对象（简化版本，返回字符串表示）
/// Convert AST to Python object (simplified version, returns string representation)
fn ast_to_pyobject(py: Python, ast: &[grammar::core::GrammarElement]) -> PyObject {
    // 简化实现：将AST转换为字符串表示
    // Simplified implementation: convert AST to string representation
    let mut result = String::new();
    for element in ast {
        result.push_str(&format!("{:?}", element));
    }
    result.to_object(py)
}

