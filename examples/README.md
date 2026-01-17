# Aevolang 示例代码 / Example Code

本目录包含各种 Aevolang 示例程序，帮助你学习和理解语言特性。

## 示例文件 / Example Files

### basic.aevo
基础语法示例，包括：
- 基本运算（加减乘除）
- 变量绑定
- 条件表达式
- 简单函数定义
- 比较操作符

### functions.aevo
函数相关示例，包括：
- 递归函数（阶乘、斐波那契）
- 高阶函数概念
- 辅助函数（判断奇偶、绝对值等）

### advanced.aevo
高级特性示例，包括：
- 嵌套函数
- 复杂条件逻辑
- 数学函数实现
- 字符串操作

### modules.aevo
模块系统示例，包括：
- 导入模块（import）
- 调用模块函数（带命名空间）

## 如何运行示例 / How to Run Examples

### 方式1: 使用演示程序

运行主程序查看内置示例：
```bash
cargo run
```

### 方式2: 在代码中加载文件

```rust
use std::fs;
use aevo::parser::AdaptiveParser;
use aevo::runtime::Interpreter;

let code = fs::read_to_string("examples/basic.aevo")?;
let parser = AdaptiveParser::new(true);
let mut interpreter = Interpreter::new();

match parser.parse(&code) {
    Ok(ast) => {
        match interpreter.execute(&ast) {
            Ok(value) => println!("Result: {}", value),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
    Err(e) => eprintln!("Parse error: {:?}", e),
}
```

## 学习路径 / Learning Path

建议按以下顺序学习：

1. **basic.aevo** - 从基础语法开始
2. **functions.aevo** - 学习函数定义和递归
3. **advanced.aevo** - 探索高级特性

## 创建自己的示例 / Create Your Own Examples

1. 创建 `.aevo` 文件
2. 使用 Aevolang 语法编写代码
3. 在代码中加载并执行

示例模板：
```lisp
; 你的程序注释
(def myFunction (x)
    ; 函数体
    (+ x 1))

; 调用函数
(myFunction 5)
```

## 提示 / Tips

- 使用分号 `;` 添加注释
- 保持代码格式清晰，适当缩进
- 使用有意义的变量和函数名
- 从简单开始，逐步增加复杂度

