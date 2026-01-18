# Evo-lang 高级特性 / Advanced Features

## 函数作用域 / Function Scope

函数定义会创建新的作用域，参数只在函数体内可见：

```lisp
(let x 10
    (def addX (y) (+ x y))  ; x 来自外层作用域
    (addX 5))               ; 结果: 15
```

## 递归 / Recursion

Evo-lang 完全支持递归函数调用：

```lisp
(def factorial (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1)))))
```

## 嵌套函数 / Nested Functions

可以在函数内部定义辅助函数：

```lisp
(def outer (x)
    (def inner (y) (+ x y))
    (inner 5))
```

## 闭包 / Closures

函数可以捕获外部作用域的变量：

```lisp
(let multiplier 3
    (def multiply (x) (* multiplier x))
    (multiply 4))  ; 结果: 12
```

## 函数组合 / Function Composition

可以组合多个函数：

```lisp
(def square (x) (* x x))
(def addOne (x) (+ x 1))

; 先平方再加1
(addOne (square 3))  ; 结果: 10
```

## 错误处理模式 / Error Handling Patterns

### 防御性编程

```lisp
(def safeDivide (a b)
    (if (= b 0)
        null
        (/ a b)))
```

### 参数验证

```lisp
(def factorial (n)
    (if (< n 0)
        null  ; 或抛出错误
        (if (= n 0)
            1
            (* n (factorial (- n 1))))))
```

## 性能优化建议 / Performance Tips

1. **避免不必要的递归**
   - 对于简单计算，使用迭代更高效
   - 考虑尾递归优化（未来支持）

2. **缓存计算结果**
   - 对于重复计算，考虑使用记忆化（未来支持）

3. **减少函数调用开销**
   - 内联简单函数（未来支持）

## 自然语言理解（NLU） / Natural Language Understanding

Evo-lang 现在支持基于规则的自然语言理解，可以将自然语言输入转换为代码结构。

### 函数定义 / Function Definition

**中文示例：**
```
"定义一个函数叫add，参数是x和y，返回x加y"
"定义一个函数multiply，参数是a和b，a乘以b"
```

**英文示例：**
```
"define a function called add that takes x and y and returns x plus y"
"create a function multiply with parameters a and b that returns a times b"
```

### 变量定义 / Variable Definition

**中文示例：**
```
"定义一个变量x等于10"
"定义一个变量count等于二十三"
```

**英文示例：**
```
"let variable x be 5"
"define a variable y equals 20"
```

### 操作表达式 / Operation Expressions

**中文示例：**
```
"3 加 5"
"8 乘以 7"
"15 除以 3"
"100 减去 25"
```

**英文示例：**
```
"10 plus 20"
"5 times 3"
"20 divided by 4"
```

### 使用NLU / Using NLU

在 Rust 代码中使用：

```rust
use evo::parser::NLUParser;
use evo::parser::AdaptiveParser;
use evo::runtime::Interpreter;

let nlu_parser = NLUParser::new_rule_based();
let code_parser = AdaptiveParser::new(true);
let mut interpreter = Interpreter::new();

// 解析自然语言
match nlu_parser.parse("定义一个函数add，参数是x和y，x加y") {
    Ok(parsed_intent) => {
        println!("识别意图: {:?}", parsed_intent.intent_type);
        println!("置信度: {:.2}", parsed_intent.confidence);
        // 将代码结构转换为可执行代码...
    }
    Err(e) => eprintln!("错误: {:?}", e),
}
```

### NLU特性 / NLU Features

- ✅ 支持中英文函数定义识别
- ✅ 支持中英文变量定义识别
- ✅ 支持中英文操作表达式识别
- ✅ 支持条件表达式（如果/否则、if/else）
- ✅ 支持多步骤表达式（然后/并且/then）
- ✅ 支持中文数字解析（如"二十三"、"一百"等）
- ✅ 自动提取函数名、参数、函数体
- ✅ 自动提取变量名和值
- ✅ 自动生成代码结构

### NLU限制 / NLU Limitations

当前版本基于规则匹配，有以下限制：

1. **模式匹配**：需要遵循特定的语言模式
2. **复杂表达式**：支持多步骤和条件，但复杂嵌套表达式仍有限
3. **上下文理解**：不支持多轮对话和上下文记忆
4. **错误处理**：对于无法识别的输入，会返回默认值或错误

未来版本将集成机器学习模型以提升理解能力。

## 进化引擎自举 / Evolution Bootstrapping

进化引擎开始使用 Evo-lang 模块来生成语法规则变体，实现“语言用语言定义规则”的自举能力。

```lisp
; 进化规则模块（modules/evolution.evo）
(import "evolution")
(evolution.bootstrap_rules)
```

## 数据结构 / Data Structures

### 列表操作 ✅ 已实现

Evo-lang 现在支持列表数据结构及其基本操作：

```lisp
; 创建列表
(list 1 2 3)
(list "a" "b" "c")

; 列表操作
(list-length (list 1 2 3))           ; 获取长度
(list-get (list 10 20 30) 1)         ; 获取元素
(list-set (list 1 2 3) 0 10)         ; 设置元素
(list-append (list 1 2) 3)           ; 追加元素
(+ (list 1 2) (list 3 4))           ; 列表连接
```

### 字典操作 ✅ 已实现

Evo-lang 现在支持字典（键值对）数据结构：

```lisp
; 创建字典
(dict "name" "Evo-lang" "version" "1.0")
(dict "x" 1 "y" 2)

; 字典操作
(dict-get (dict "name" "Evo") "name")    ; 获取值
(dict-set (dict "x" 1) "y" 2)            ; 设置键值
(dict-keys (dict "a" 1 "b" 2))           ; 获取所有键
(dict-values (dict "a" 1 "b" 2))         ; 获取所有值
(dict-has (dict "name" "Evo") "name")   ; 检查键是否存在
```

## 模块系统 / Module System ✅ 已实现

Evo-lang 现在支持模块导入和命名空间调用：

```lisp
; 导入模块
(import "math")

; 调用模块函数
(math.add 3 4)
(math.square 5)

; 使用别名
(import "math" "m")
(m.square 6)
```

## 未来特性 / Future Features

以下特性正在开发中：

1. **高级列表操作**
   ```lisp
   (map square (list 1 2 3))
   (filter isEven (list 1 2 3))
   (reduce + 0 (list 1 2 3))
   ```

2. **模式匹配**
   ```lisp
   (match x
       (0 "zero")
       (1 "one")
       (_ "other"))
   ```

3. **类型系统**
   ```lisp
   (def add (x: Int y: Int) -> Int
       (+ x y))
   ```

4. **模块系统**
   ```lisp
   (import math)
   (math.sqrt 16)
   ```

5. **自然语言编程** ✅ 已实现基础版本
   - 支持基于规则的意图识别
   - 支持中英文自然语言输入
   - 自动生成代码结构

## 调试技巧 / Debugging Tips

1. **使用 print 函数**
   ```lisp
   (print "Value of x:" x)
   ```

2. **分步执行**
   - 将复杂表达式拆分为多个步骤
   - 逐步验证每个部分

3. **检查类型**
   - 确保操作符的参数类型正确
   - 注意整数和浮点数的区别

## 常见陷阱 / Common Pitfalls

1. **括号不匹配**
   ```lisp
   ; 错误
   (+ 1 2
   
   ; 正确
   (+ 1 2)
   ```

2. **变量作用域**
   ```lisp
   ; 错误: x 未定义
   (let y 10 (+ x y))
   
   ; 正确
   (let x 5 (let y 10 (+ x y)))
   ```

3. **函数参数数量**
   ```lisp
   ; 错误: 参数数量不匹配
   (def add (x y) (+ x y))
   (add 1 2 3)
   
   ; 正确
   (add 1 2)
   ```

