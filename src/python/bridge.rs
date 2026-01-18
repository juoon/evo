// Python桥接 / Python bridge
// 实现Evo-lang与Python之间的互操作
// Implements interoperability between Evo-lang and Python

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyModule as PyModuleType, PyTuple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Python桥接器 / Python bridge
pub struct PyBridge {
    /// 是否已初始化 / Whether initialized
    initialized: bool,
}

impl PyBridge {
    /// 创建新桥接器 / Create new bridge
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// 初始化桥接器 / Initialize bridge
    pub fn initialize(&mut self) -> Result<(), PyBridgeError> {
        // PyO3会自动初始化Python解释器（通过auto-initialize特性）
        // PyO3 automatically initializes Python interpreter (via auto-initialize feature)
        Python::with_gil(|_py| {
            // 验证Python解释器可用 / Verify Python interpreter is available
            Ok::<(), PyBridgeError>(())
        })?;
        self.initialized = true;
        Ok(())
    }

    /// 导入Python模块 / Import Python module
    pub fn import_module(&self, module_name: &str) -> Result<PyModuleInfo, PyBridgeError> {
        if !self.initialized {
            return Err(PyBridgeError::NotInitialized);
        }

        Python::with_gil(|py| {
            match PyModuleType::import_bound(py, module_name) {
                Ok(module) => {
                    // 尝试获取模块中的函数列表（可选）
                    // Try to get function list from module (optional)
                    let mut functions = Vec::new();

                    // 尝试获取模块的 __dict__ 来查找函数
                    // Try to get module's __dict__ to find functions
                    if let Ok(dict) = module.getattr("__dict__") {
                        if let Ok(py_dict) = dict.downcast::<PyDict>() {
                            for (key, _value) in py_dict.iter() {
                                if let Ok(key_str) = key.extract::<String>() {
                                    // 简单检查：如果值是可调用的，认为是函数
                                    // Simple check: if value is callable, consider it a function
                                    if let Ok(Some(value)) = py_dict.get_item(key_str.as_str()) {
                                        if value.is_callable() {
                                            functions.push(key_str);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Ok(PyModuleInfo {
                        name: module_name.to_string(),
                        functions,
                    })
                }
                Err(e) => Err(PyBridgeError::PythonError(format!(
                    "Failed to import module '{}': {}",
                    module_name, e
                ))),
            }
        })
    }

    /// 调用Python函数 / Call Python function
    pub fn call_function(
        &self,
        module: &str,
        function: &str,
        args: &[PyValue],
    ) -> Result<PyValue, PyBridgeError> {
        if !self.initialized {
            return Err(PyBridgeError::NotInitialized);
        }

        Python::with_gil(|py| {
            // 导入模块
            let py_module = PyModuleType::import_bound(py, module).map_err(|e| {
                PyBridgeError::PythonError(format!("Failed to import module '{}': {}", module, e))
            })?;

            // 获取函数
            let py_func = py_module.getattr(function).map_err(|e| {
                PyBridgeError::PythonError(format!("Failed to get function '{}': {}", function, e))
            })?;

            // 转换参数
            let py_args: Vec<_> = args
                .iter()
                .map(|v| py_value_to_pyobject_bound(py, v))
                .collect::<Result<Vec<_>, _>>()?;

            // 调用函数
            let py_tuple = PyTuple::new_bound(py, py_args.iter().map(|a| a.as_ref()));
            let result = py_func.call1(py_tuple).map_err(|e| {
                PyBridgeError::PythonError(format!("Failed to call function '{}': {}", function, e))
            })?;

            // 转换返回值
            pyobject_to_py_value_bound(py, &result)
        })
    }

    /// 执行Python代码 / Execute Python code
    pub fn execute_code(&self, code: &str) -> Result<PyValue, PyBridgeError> {
        if !self.initialized {
            return Err(PyBridgeError::NotInitialized);
        }

        Python::with_gil(|py| {
            // 使用 compile 和 exec 来执行代码
            // Use compile and exec to execute code
            match py.run_bound(code, None, None) {
                Ok(_) => Ok(PyValue::None),
                Err(e) => Err(PyBridgeError::PythonError(format!(
                    "Python execution error: {}",
                    e
                ))),
            }
        })
    }

    /// 执行Python表达式并返回值 / Execute Python expression and return value
    pub fn eval(&self, expression: &str) -> Result<PyValue, PyBridgeError> {
        if !self.initialized {
            return Err(PyBridgeError::NotInitialized);
        }

        Python::with_gil(|py| match py.eval_bound(expression, None, None) {
            Ok(result) => pyobject_to_py_value_bound(py, &result),
            Err(e) => Err(PyBridgeError::PythonError(format!(
                "Python eval error: {}",
                e
            ))),
        })
    }
}

impl Default for PyBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Python模块信息 / Python module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyModuleInfo {
    /// 模块名称 / Module name
    pub name: String,
    /// 可用函数 / Available functions
    pub functions: Vec<String>,
}

/// Python值 / Python value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PyValue {
    /// 整数 / Integer
    Int(i64),
    /// 浮点数 / Float
    Float(f64),
    /// 字符串 / String
    String(String),
    /// 列表 / List
    List(Vec<PyValue>),
    /// 字典 / Dictionary
    Dict(HashMap<String, PyValue>),
    /// 布尔值 / Boolean
    Bool(bool),
    /// None
    None,
}

impl PyValue {
    /// 从Evo-lang Value转换 / Convert from Evo-lang Value
    pub fn from_evo_value(value: &crate::runtime::interpreter::Value) -> Self {
        match value {
            crate::runtime::interpreter::Value::Int(i) => PyValue::Int(*i),
            crate::runtime::interpreter::Value::Float(f) => PyValue::Float(*f),
            crate::runtime::interpreter::Value::String(s) => PyValue::String(s.clone()),
            crate::runtime::interpreter::Value::Bool(b) => PyValue::Bool(*b),
            crate::runtime::interpreter::Value::Null => PyValue::None,
            crate::runtime::interpreter::Value::Lambda { params, .. } => {
                PyValue::String(format!("<lambda({})>", params.join(", ")))
            }
            crate::runtime::interpreter::Value::List(list) => {
                PyValue::List(list.iter().map(|v| PyValue::from_evo_value(v)).collect())
            }
            crate::runtime::interpreter::Value::Dict(dict) => PyValue::Dict(
                dict.iter()
                    .map(|(k, v)| (k.clone(), PyValue::from_evo_value(v)))
                    .collect(),
            ),
        }
    }

    /// 转换为Evo-lang Value / Convert to Evo-lang Value
    pub fn to_evo_value(&self) -> crate::runtime::interpreter::Value {
        match self {
            PyValue::Int(i) => crate::runtime::interpreter::Value::Int(*i),
            PyValue::Float(f) => crate::runtime::interpreter::Value::Float(*f),
            PyValue::String(s) => crate::runtime::interpreter::Value::String(s.clone()),
            PyValue::Bool(b) => crate::runtime::interpreter::Value::Bool(*b),
            PyValue::None => crate::runtime::interpreter::Value::Null,
            PyValue::List(list) => crate::runtime::interpreter::Value::List(
                list.iter().map(|v| v.to_evo_value()).collect(),
            ),
            PyValue::Dict(dict) => crate::runtime::interpreter::Value::Dict(
                dict.iter()
                    .map(|(k, v)| (k.clone(), v.to_evo_value()))
                    .collect(),
            ),
        }
    }
}

/// Python桥接错误 / Python bridge error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PyBridgeError {
    /// 未初始化 / Not initialized
    NotInitialized,
    /// 未实现 / Not implemented
    NotImplemented,
    /// Python错误 / Python error
    PythonError(String),
    /// 类型转换错误 / Type conversion error
    TypeConversionError(String),
}

impl std::fmt::Display for PyBridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PyBridgeError::NotInitialized => write!(f, "Python bridge not initialized"),
            PyBridgeError::NotImplemented => write!(f, "Feature not implemented"),
            PyBridgeError::PythonError(msg) => write!(f, "Python error: {}", msg),
            PyBridgeError::TypeConversionError(msg) => write!(f, "Type conversion error: {}", msg),
        }
    }
}

impl std::error::Error for PyBridgeError {}

impl From<PyErr> for PyBridgeError {
    fn from(err: PyErr) -> Self {
        PyBridgeError::PythonError(err.to_string())
    }
}

/// 将PyValue转换为Bound<PyAny> / Convert PyValue to Bound<PyAny>
fn py_value_to_pyobject_bound<'py>(
    py: Python<'py>,
    value: &PyValue,
) -> Result<Bound<'py, PyAny>, PyBridgeError> {
    match value {
        PyValue::Int(i) => Ok(i.to_object(py).into_bound(py)),
        PyValue::Float(f) => Ok(f.to_object(py).into_bound(py)),
        PyValue::String(s) => Ok(s.to_object(py).into_bound(py)),
        PyValue::Bool(b) => Ok(b.to_object(py).into_bound(py)),
        PyValue::None => Ok(py.None().into_bound(py)),
        PyValue::List(list) => {
            let py_list = PyList::empty_bound(py);
            for item in list {
                let item_bound = py_value_to_pyobject_bound(py, item)?;
                py_list.append(item_bound)?;
            }
            Ok(py_list.as_any().clone())
        }
        PyValue::Dict(dict) => {
            let py_dict = PyDict::new_bound(py);
            for (k, v) in dict {
                let value_bound = py_value_to_pyobject_bound(py, v)?;
                py_dict.set_item(k, value_bound)?;
            }
            Ok(py_dict.as_any().clone())
        }
    }
}

/// 将Bound<PyAny>转换为PyValue / Convert Bound<PyAny> to PyValue
fn pyobject_to_py_value_bound<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
) -> Result<PyValue, PyBridgeError> {
    // 尝试提取为各种类型
    // Try to extract as various types

    // 整数
    if let Ok(i) = obj.extract::<i64>() {
        return Ok(PyValue::Int(i));
    }

    // 浮点数
    if let Ok(f) = obj.extract::<f64>() {
        return Ok(PyValue::Float(f));
    }

    // 字符串
    if let Ok(s) = obj.extract::<String>() {
        return Ok(PyValue::String(s));
    }

    // 布尔值
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(PyValue::Bool(b));
    }

    // None
    if obj.is_none() {
        return Ok(PyValue::None);
    }

    // 列表
    if let Ok(py_list) = obj.downcast::<PyList>() {
        let mut list = Vec::new();
        for item in py_list.iter() {
            list.push(pyobject_to_py_value_bound(py, &item)?);
        }
        return Ok(PyValue::List(list));
    }

    // 字典
    if let Ok(py_dict) = obj.downcast::<PyDict>() {
        let mut dict = HashMap::new();
        for (key, value) in py_dict.iter() {
            let key_str = key.extract::<String>().map_err(|_| {
                PyBridgeError::TypeConversionError("Dict key must be string".to_string())
            })?;
            dict.insert(key_str, pyobject_to_py_value_bound(py, &value)?);
        }
        return Ok(PyValue::Dict(dict));
    }

    // 如果无法转换，返回字符串表示
    // If cannot convert, return string representation
    Ok(PyValue::String(format!("{:?}", obj)))
}
