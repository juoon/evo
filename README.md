# Aevolang - 自进化编程语言 / Self-evolving Programming Language

一个自进化的编程语言，能够根据使用和需求自我进化，终极目标是理解人类思想，促进人类与智能生命和谐共生。

A self-evolving programming language that can evolve based on usage and needs, with the ultimate goal of understanding human thoughts and promoting harmonious coexistence between humans and intelligent life.

## 项目结构 / Project Structure

```
src/
├── main.rs              # 入口程序 / Entry point
├── grammar/             # 语法定义系统 / Grammar definition system
│   ├── mod.rs
│   ├── core.rs          # 核心语法 / Core grammar
│   ├── rule.rs          # 语法规则定义 / Grammar rule definition
│   └── self_desc.rs     # 自描述语法 / Self-describing syntax
├── parser/              # 解析器 / Parser
│   ├── mod.rs
│   ├── adaptive.rs      # 自适应解析器 / Adaptive parser
│   └── nlu.rs           # 自然语言解析器 / Natural Language Understanding parser
├── evolution/           # 进化引擎 / Evolution engine
│   ├── mod.rs
│   ├── engine.rs        # 进化引擎核心 / Evolution engine core
│   ├── tracker.rs       # 进化记录器 / Evolution tracker
│   └── knowledge.rs     # 知识图谱 / Knowledge graph
├── runtime/             # 运行时 / Runtime
│   ├── mod.rs
│   ├── interpreter.rs   # 解释器 / Interpreter
│   ├── jit.rs           # JIT编译器 / JIT compiler
│   ├── jit_interpreter.rs # JIT解释器 / JIT interpreter
│   └── mode.rs          # 执行模式选择 / Execution mode selection
├── python/              # Python兼容层 / Python compatibility layer
│   ├── mod.rs
│   └── bridge.rs        # Python桥接 / Python bridge
└── poetry/              # 诗歌理解模块 / Poetry understanding module
    ├── mod.rs
    ├── parser.rs        # 诗歌解析 / Poetry parser
    └── emotion.rs       # 情感理解 / Emotion understanding
modules/                 # 模块目录 / Module directory
```

## 核心特性 / Core Features

### 1. 自进化机制 / Self-evolution Mechanism
- 语法和语义的动态扩展 / Dynamic extension of syntax and semantics
- 基于使用模式的自动优化 / Automatic optimization based on usage patterns
- 进化历史记录和回滚 / Evolution history recording and rollback

### 2. 自然语言编程 / Natural Language Programming
- 支持自然语言输入 / Support for natural language input
- 意图识别和代码生成 / Intent recognition and code generation
- 多模型支持（本地/云） / Multi-model support (local/cloud)

### 3. 三层语法系统 / Three-layer Grammar System
- 基础层：最小核心语法（类似Lisp S-expression）/ Base layer: Minimal core grammar (similar to Lisp S-expression)
- 扩展层：用户定义的语法规则 / Extension layer: User-defined grammar rules
- IR层：中间表示（用于AI分析和优化）/ IR layer: Intermediate representation (for AI analysis and optimization)

### 4. 混合执行模式 / Hybrid Execution Mode
- 解释模式：快速原型开发 / Interpreted mode: Fast prototyping
- 编译模式：高性能执行 / Compiled mode: High-performance execution
- JIT模式：自适应优化 / JIT mode: Adaptive optimization

### 5. 诗歌理解能力 / Poetry Understanding Capability ✅ 已实现
- 以《静夜思》为起点的情感理解 / Emotion understanding starting with "Quiet Night Thoughts"
- **主题提取** - 基于关键词和情感分析的智能主题识别 / Theme extraction - intelligent theme recognition
- **意象提取** - 自动识别诗歌中的意象元素及其含义 / Imagery extraction - automatic recognition of imagery elements
- **情感分析** - 丰富的情感词典，支持多种情感类型 / Emotion analysis - rich emotion dictionary supporting multiple emotion types

## 快速开始 / Quick Start

### 安装 / Installation

确保已安装 Rust (1.70+):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Python互操作 / Python Interoperability

Aevolang支持Python互操作，可以从Python代码中调用Aevolang功能。

Aevolang supports Python interoperability, allowing you to call Aevolang functions from Python code.

**安装Python模块 / Install Python Module:**

```bash
# 安装maturin
pip install maturin

# 构建并安装Python模块
maturin develop
```

**使用示例 / Usage Example:**

```python
import aevo

# 执行Aevolang代码
result = aevo.execute("(+ 1 2)")
print(result)  # 输出: 3

# 使用解释器类
interpreter = aevo.AevoInterpreter()
interpreter.execute("(def add (x y) (+ x y))")
result = interpreter.execute("(add 3 4)")
print(result)  # 输出: 7
```

更多信息请查看 [python/README.md](python/README.md)。

For more information, see [python/README.md](python/README.md).

### 运行示例 / Run Examples

```bash
# 克隆仓库
git clone <repository-url>
cd aevo

# 运行演示程序
cargo run
```

程序将演示：
1. 《静夜思》的解析和理解 / Parsing and understanding of "Quiet Night Thoughts"
2. 语法定义示例 / Grammar definition examples
3. 进化引擎演示 / Evolution engine demonstration
4. 解析器功能演示 / Parser functionality demo
5. 解释器执行演示 / Interpreter execution demo

### 编写第一个程序 / Your First Program

创建一个 `hello.aevo` 文件：

```lisp
; 定义问候函数
(def greet (name) (+ "Hello, " name))

; 使用函数
(greet "Aevolang")
```

在 Rust 代码中执行：

```rust
use aevo::parser::AdaptiveParser;
use aevo::runtime::Interpreter;

let parser = AdaptiveParser::new(true);
let mut interpreter = Interpreter::new();

let code = r#"
    (def greet (name) (+ "Hello, " name))
    (greet "Aevolang")
"#;

match parser.parse(code) {
    Ok(ast) => {
        match interpreter.execute(&ast) {
            Ok(value) => println!("{}", value),  // 输出: Hello, Aevolang
            Err(e) => eprintln!("执行错误: {:?}", e),
        }
    }
    Err(e) => eprintln!("解析错误: {:?}", e),
}
```

## 使用示例 / Usage Examples

### 基本运算 / Basic Operations

```lisp
(+ 1 2)           ; 3
(* 3 4)           ; 12
(let x 10 (+ x 5)) ; 15
```

### 函数定义 / Function Definition

```lisp
(def add (x y) (+ x y))
(add 3 4)  ; 7
```

### 递归函数 / Recursive Functions

```lisp
(def factorial (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1)))))

(factorial 5)  ; 120
```

### 条件表达式 / Conditional Expressions

```lisp
(if (> x 0) x (- x))  ; 返回 x 的绝对值
```

### 列表操作 / List Operations

```lisp
; 创建列表 / Create list
(list 1 2 3 4 5)

; 获取列表元素 / Get list element
(list-get (list 10 20 30) 1)  ; 返回 20

; 追加元素 / Append element
(list-append (list 1 2) 3)  ; 返回 [1, 2, 3]

; 获取列表长度 / Get list length
(list-length (list 1 2 3))  ; 返回 3

; 列表连接 / List concatenation
(+ (list 1 2) (list 3 4))  ; 返回 [1, 2, 3, 4]
```

### 字典操作 / Dictionary Operations

```lisp
; 创建字典 / Create dictionary
(dict "name" "Aevolang" "version" "1.0")

; 获取字典值 / Get dictionary value
(dict-get (dict "name" "Aevolang") "name")  ; 返回 "Aevolang"

; 设置字典值 / Set dictionary value
(dict-set (dict "x" 1) "y" 2)  ; 返回 {"x": 1, "y": 2}

; 获取所有键 / Get all keys
(dict-keys (dict "a" 1 "b" 2))  ; 返回 ["a", "b"]

; 检查键是否存在 / Check if key exists
(dict-has (dict "name" "Aevolang") "name")  ; 返回 true
```

### 模块系统 / Module System

```lisp
; 导入模块 / Import module
(import "math")

; 调用模块函数 / Call module function
(math.add 3 4)      ; 返回 7
(math.square 5)     ; 返回 25
```

更多示例请查看 [examples/](../examples/) 目录。

## 文档 / Documentation

- [快速入门指南](docs/getting-started.md) - 学习如何使用 Aevolang
- [语法参考](docs/syntax-reference.md) - 完整的语法文档
- [高级特性](docs/advanced-features.md) - 深入了解语言特性
- [示例代码](../examples/) - 各种示例程序

## 设计哲学 / Design Philosophy

1. **渐进式发展** / Progressive Development: 从简单核心开始，逐步扩展
2. **自描述优先** / Self-description First: 尽早实现用语言自身描述自身的能力
3. **以人为本** / Human-centered: 以理解人类思想为终极目标
4. **和谐共生** / Harmonious Coexistence: 设计时考虑人类与AI的协作模式
5. **开放进化** / Open Evolution: 记录所有进化过程，支持回滚和学习

## 技术栈 / Technology Stack

- **实现语言** / Implementation Language: Rust
- **序列化** / Serialization: serde, serde_json
- **时间处理** / Time Handling: chrono
- **唯一标识** / Unique Identifiers: uuid

## 状态 / Status

当前版本实现了核心框架和基础功能：
- ✅ 项目基础结构和核心数据类型定义
- ✅ 基础语法系统和《静夜思》解析示例
- ✅ 自描述语法机制
- ✅ **完整的解析器实现** - 支持 S-expression 语法解析
- ✅ **完整的解释器实现** - 支持代码执行、函数调用、递归等
- ✅ **进化引擎核心功能** - 自举规则加载、知识图谱、进化预测
  - 从Aevolang模块加载引导规则
  - 知识图谱构建：实体提取、关系挖掘、模式发现
  - 进化预测：基于历史模式和目标的智能预测
- ✅ **进化规则自举** - 使用Aevolang模块驱动进化规则生成
- ✅ **简单NLU系统** - 基于规则的意图识别，支持中英文自然语言输入
- ✅ **Python互操作** - 完整的PyO3集成，支持从Python调用Aevolang
- ✅ **JIT编译器** - 热点代码检测和优化，支持常量折叠等优化技术
- ✅ **列表和字典** - 完整的数据结构支持，包括创建、访问、修改和内置函数

### 已实现功能 / Implemented Features

- ✅ **词法分析** - 支持数字、字符串、标识符、操作符
- ✅ **语法分析** - 完整的 S-expression 解析器
- ✅ **表达式求值** - 支持算术、比较、逻辑运算
- ✅ **变量绑定** - `let` 支持作用域管理
- ✅ **函数定义** - `def` 和 `function` 关键字
- ✅ **函数调用** - 支持用户定义函数和递归
- ✅ **条件表达式** - `if` 条件分支
- ✅ **类型系统** - Int, Float, String, Bool, Null, List, Dict
- ✅ **自然语言理解（NLU）** - 基于规则的意图识别
  - 支持中英文函数定义识别
  - 支持中英文变量定义识别
  - 支持中英文操作表达式识别
  - 支持条件表达式（如果/否则、if/else）
  - 支持多步骤表达式（然后/并且/then）
  - 支持中文数字解析（如"二十三"、"一百"等）
  - 自动生成代码结构
- ✅ **Python互操作** - PyO3集成
  - Python模块导出（`aevo`）
  - 支持从Python调用Aevolang解析器和解释器
  - 提供`execute`、`eval`、`parse`函数接口
  - 提供`AevoInterpreter`和`AevoParser`类
  - 完整的类型转换支持（Int, Float, String, Bool, None）
- ✅ **JIT编译器** - 热点代码优化
  - 热点代码自动检测（基于执行次数阈值）
  - 代码执行统计和性能分析
  - 常量折叠优化（编译时计算常量表达式）
  - 热点代码缓存和优化执行
  - 可配置的编译阈值
  - JIT统计信息查询
- ✅ **列表和字典支持** - 数据结构操作
  - 列表字面量：`(list item1 item2 ...)` 或 `(vec item1 item2 ...)`
  - 字典字面量：`(dict key1 value1 key2 value2 ...)` 或 `(map key1 value1 ...)`
  - 列表操作：`list-get`, `list-set`, `list-append`, `list-length`
  - 字典操作：`dict-get`, `dict-set`, `dict-keys`, `dict-values`, `dict-has`
  - 列表连接：使用 `+` 操作符连接两个列表
- ✅ **模块系统** - 导入与命名空间
  - 导入模块：`(import "module")` 或 `(import "module" "alias")`
  - 模块命名空间调用：`(module.function ...)`
  - 默认搜索路径：`modules/`, `examples/`, 当前目录
- ✅ **标准库模块** - 用Aevolang实现的核心功能
  - 标准库：`(import "std")` 提供常用工具函数
  - 数学函数：`std.abs`, `std.max`, `std.min`, `std.factorial`
  - 逻辑函数：`std.and`, `std.or`, `std.not`
  - 增强自举能力：更多功能用语言本身实现

### 测试状态 / Test Status

所有核心功能已通过测试：
- ✅ 基本运算（加减乘除）
- ✅ 变量绑定和作用域
- ✅ 条件表达式
- ✅ 函数定义和调用
- ✅ 递归函数（阶乘、斐波那契等）
- ✅ NLU自然语言理解（中英文函数定义、变量定义、操作表达式）
- ✅ Python互操作（模块导入、函数调用、类型转换）
- ✅ JIT编译器（热点检测、常量折叠、代码优化）
- ✅ 列表和字典数据结构（创建、访问、修改、内置函数）
- ✅ 模块系统（import、命名空间调用、模块文件加载）
- ✅ 模块系统（import、命名空间、示例模块）

## 下一步 / Next Steps

1. ✅ ~~实现完整的解释器功能~~ - 已完成
2. ✅ ~~实现简单NLU - 基于规则的意图识别（不依赖外部模型）~~ - 已完成
3. ✅ ~~集成PyO3实现Python互操作~~ - 已完成
4. ✅ ~~实现JIT编译器 - 热点代码优化~~ - 已完成
5. ✅ ~~增强NLU系统 - 支持更复杂的自然语言表达式和上下文理解~~ - 已完成
6. 集成NLU模型（本地轻量模型或云API）- 提升理解能力
7. ✅ ~~完善进化引擎的学习和预测功能~~ - 已完成
   - 知识图谱实体提取和关系挖掘
   - 进化模式挖掘与预测
   - 整合知识图谱到进化引擎
8. ✅ ~~添加列表和数据结构支持~~ - 已完成
9. ✅ ~~实现模块系统~~ - 已完成
10. ✅ ~~进化规则自举 - 用Aevolang模块驱动规则生成~~ - 已完成
11. ✅ ~~诗歌理解与进化引擎结合 - 从理解诗歌中学习并进化~~ - 已完成
    - 将情感、主题、意象作为知识图谱节点
    - 从诗歌理解中生成语法规则
    - 整合诗歌解析器到进化引擎
12. ✅ ~~增强知识图谱推理能力 - 智能相似度计算和自我反思~~ - 已完成
    - 实现规则相似度计算（名称、模式、产生式多维度）
    - 实现实体相似度查找
    - 实现进化引擎自我反思机制
    - 评估进化效果和知识图谱丰富度
13. ✅ ~~进化谱系构建与回滚机制 - 完整的历史追踪能力~~ - 已完成
    - 实现父事件查找逻辑（基于规则相似度和时间顺序）
    - 实现进化回滚机制：rollback_to_event()
    - 实现谱系树结构查询
    - 实现祖先链和后代事件查询
14. ✅ ~~标准库模块（std.aevo）- 增强自举能力~~ - 已完成
    - 创建标准库模块，用Aevolang实现常用工具函数
    - 数学函数：abs、max、min、factorial
    - 逻辑函数：and、or、not
    - 工具函数：identity
    - 为未来高级函数（map、filter、reduce）预留接口
15. ✅ ~~从诗歌理解到代码生成 - 完整的情感到代码映射~~ - 已完成
    - 实现generate_code_from_poetry()方法
    - 基于情感生成代码结构（思乡、宁静、孤独等）
    - 基于主题生成函数定义
    - 基于意象生成数据结构
    - 增强诗歌理解的深度和代码生成能力
16. ✅ ~~代码解释功能 - 从代码到自然语言~~ - 已完成
    - 实现CodeExplainer：将代码结构转换为自然语言
    - 支持函数定义、变量定义、条件表达式等解释
    - 支持中英文双语解释
    - 增强代码可读性和理解能力
17. ✅ ~~代码分析和优化 - 自动识别代码模式和优化机会~~ - 已完成
    - 实现CodeAnalyzer：分析代码模式和结构
    - 识别长函数、复杂表达式、深度嵌套等模式
    - 生成优化建议（简化、重构、提取函数等）
    - 计算代码复杂度和统计数据
    - 增强代码质量提升能力
18. ✅ ~~代码自动重构 - 基于分析结果自动改进代码~~ - 已完成
    - 实现CodeRefactorer：根据分析结果自动重构代码
    - 简化表达式：常量折叠优化
    - 减少嵌套：扁平化代码结构
    - 提取函数：将长函数拆分为小函数
    - 实现真正的自进化：语言能够改进自身代码
19. ✅ ~~自我进化闭环 - 语言自动分析和改进自身实现~~ - 已完成
    - 实现self_evolve()方法：自动分析并改进自身实现
    - 分析当前规则和代码结构
    - 生成改进建议并记录进化事件
    - 形成完整的自进化闭环
    - 实现真正的自我改进能力
20. ✅ ~~上下文理解 - 多轮对话和上下文管理~~ - 已完成
    - 实现ContextManager：维护对话历史和状态
    - 支持多轮对话：理解上下文引用
    - 自动解析变量和函数引用
    - 上下文感知的代码生成
    - 增强理解能力：让语言记住对话历史
21. ✅ ~~增强标准库 - 更多用Aevolang实现的核心功能~~ - 已完成
    - 添加更多列表操作函数：sum、product、reverse、contains、count
    - 添加更多数学函数：power、sqrt-approx
    - 添加范围生成函数：range、range-step
    - 添加字典操作函数：dict-has-key、dict-merge
    - 添加工具函数：repeat、when、unless
    - 增强自举能力：更多功能用语言本身实现

## 贡献 / Contributing

欢迎贡献代码！请查看 [贡献指南](CONTRIBUTING.md)（待创建）。

## 许可证 / License

待定 / To be determined

