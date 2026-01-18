# Evo-lang Python互操作 / Evo-lang Python Interoperability

本文档说明如何使用Evo-lang的Python接口。

This document explains how to use Evo-lang's Python interface.

## 构建Python模块 / Building Python Module

### 方法1：使用Maturin（推荐） / Method 1: Using Maturin (Recommended)

```bash
# 安装maturin
pip install maturin

# 在项目根目录构建并安装
maturin develop
```

### 方法2：手动构建 / Method 2: Manual Build

```bash
# 构建Rust库
cargo build --release

# Python模块文件将位于 target/release/ 目录
# Python module file will be in target/release/ directory
```

## 使用方法 / Usage

### 基本使用 / Basic Usage

```python
import evo

# 执行Evo-lang代码并返回结果字符串
# Execute Evo-lang code and return result string
result = evo.execute("(+ 1 2)")
print(result)  # 输出: 3

# 执行代码并返回Python对象
# Execute code and return Python object
result = evo.eval("(* 3 4)")
print(result)  # 输出: 12 (整数)

# 解析代码并返回AST
# Parse code and return AST
ast = evo.parse("(def add (x y) (+ x y))")
print(ast)
```

### 使用解释器类 / Using Interpreter Class

```python
import evo

# 创建解释器实例
# Create interpreter instance
interpreter = evo.EvoInterpreter()

# 执行代码
# Execute code
result = interpreter.execute("(+ 1 2)")
print(result)  # 输出: 3

# 求值代码并返回Python对象
# Eval code and return Python object
result = interpreter.eval("(* 2 3)")
print(result)  # 输出: 6 (整数)
```

### 使用解析器类 / Using Parser Class

```python
import evo

# 创建解析器实例（启用NLU）
# Create parser instance (with NLU enabled)
parser = evo.EvoParser(enable_nlu=True)

# 解析代码
# Parse code
ast = parser.parse("(def add (x y) (+ x y))")
print(ast)
```

## API参考 / API Reference

### 模块级函数 / Module-level Functions

#### `execute(code: str) -> str`

执行Evo-lang代码并返回结果字符串。

Execute Evo-lang code and return result string.

**参数 / Parameters:**
- `code`: Evo-lang代码字符串 / Evo-lang code string

**返回 / Returns:**
- 执行结果的字符串表示 / String representation of execution result

**示例 / Example:**
```python
result = evo.execute("(+ 1 2)")  # "3"
```

#### `eval(code: str) -> object`

执行Evo-lang代码并返回Python对象。

Execute Evo-lang code and return Python object.

**参数 / Parameters:**
- `code`: Evo-lang代码字符串 / Evo-lang code string

**返回 / Returns:**
- Python对象（int, float, str, bool, None） / Python object (int, float, str, bool, None)

**示例 / Example:**
```python
result = evo.eval("(* 3 4)")  # 12 (int)
```

#### `parse(code: str) -> object`

解析Evo-lang代码并返回AST（抽象语法树）。

Parse Evo-lang code and return AST (Abstract Syntax Tree).

**参数 / Parameters:**
- `code`: Evo-lang代码字符串 / Evo-lang code string

**返回 / Returns:**
- AST的Python表示 / Python representation of AST

**示例 / Example:**
```python
ast = evo.parse("(+ 1 2)")
```

### 类 / Classes

#### `EvoInterpreter`

Evo-lang解释器类。

Evo-lang interpreter class.

**方法 / Methods:**

- `execute(code: str) -> str`: 执行代码并返回结果字符串 / Execute code and return result string
- `eval(code: str) -> object`: 执行代码并返回Python对象 / Execute code and return Python object

**示例 / Example:**
```python
interpreter = evo.EvoInterpreter()
result = interpreter.execute("(+ 1 2)")
```

#### `EvoParser`

Evo-lang解析器类。

Evo-lang parser class.

**方法 / Methods:**

- `parse(code: str) -> object`: 解析代码并返回AST / Parse code and return AST

**示例 / Example:**
```python
parser = evo.EvoParser(enable_nlu=True)
ast = parser.parse("(+ 1 2)")
```

## 测试 / Testing

运行测试脚本：

Run test script:

```bash
python test_python.py
```

## 注意事项 / Notes

1. 确保已安装Python 3.7+ / Ensure Python 3.7+ is installed
2. 确保已安装PyO3兼容的Python版本 / Ensure Python version compatible with PyO3
3. 首次使用需要构建Python模块 / First use requires building Python module
4. 函数定义需要在同一解释器实例中调用 / Function definitions need to be called in the same interpreter instance

## 示例 / Examples

更多示例请查看 `test_python.py` 文件。

For more examples, see the `test_python.py` file.

