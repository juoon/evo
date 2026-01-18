# Evo-lang - 自进化编程语言 / Self-evolving Programming Language

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

Evo-lang支持Python互操作，可以从Python代码中调用Evo-lang功能。

Evo-lang supports Python interoperability, allowing you to call Evo-lang functions from Python code.

**安装Python模块 / Install Python Module:**

```bash
# 安装maturin
pip install maturin

# 构建并安装Python模块
maturin develop
```

**使用示例 / Usage Example:**

```python
import evo

# 执行Evo-lang代码
result = evo.execute("(+ 1 2)")
print(result)  # 输出: 3

# 使用解释器类
interpreter = evo.EvoInterpreter()
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
cd evo

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

创建一个 `hello.evo` 文件：

```lisp
; 定义问候函数
(def greet (name) (+ "Hello, " name))

; 使用函数
(greet "Evo-lang")
```

在 Rust 代码中执行：

```rust
use evo::parser::AdaptiveParser;
use evo::runtime::Interpreter;

let parser = AdaptiveParser::new(true);
let mut interpreter = Interpreter::new();

let code = r#"
    (def greet (name) (+ "Hello, " name))
    (greet "Evo-lang")
"#;

match parser.parse(code) {
    Ok(ast) => {
        match interpreter.execute(&ast) {
            Ok(value) => println!("{}", value),  // 输出: Hello, Evo-lang
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

### 高阶函数 / Higher-order Functions ✅ 已实现

```lisp
; 导入标准库 / Import standard library
(import "std")

; map - 对列表每个元素应用函数 / Apply function to each element
(std.map (lambda (x) (* x 2)) (list 1 2 3 4))  ; 返回 [2, 4, 6, 8]

; filter - 过滤满足条件的元素 / Filter elements matching condition
(std.filter (lambda (x) (> x 2)) (list 1 2 3 4 5))  ; 返回 [3, 4, 5]

; reduce - 将列表归约为单个值 / Reduce list to single value
(std.reduce (lambda (acc x) (+ acc x)) 0 (list 1 2 3 4))  ; 返回 10
```

**注意**：这些高阶函数既可以通过内置函数调用（`map`、`filter`、`reduce`），也可以通过标准库调用（`std.map`、`std.filter`、`std.reduce`）。

**Note**: These higher-order functions can be called both as built-in functions (`map`, `filter`, `reduce`) and through the standard library (`std.map`, `std.filter`, `std.reduce`).
```

### 字典操作 / Dictionary Operations

```lisp
; 创建字典 / Create dictionary
(dict "name" "Evo-lang" "version" "1.0")

; 获取字典值 / Get dictionary value
(dict-get (dict "name" "Evo-lang") "name")  ; 返回 "Evo-lang"

; 设置字典值 / Set dictionary value
(dict-set (dict "x" 1) "y" 2)  ; 返回 {"x": 1, "y": 2}

; 获取所有键 / Get all keys
(dict-keys (dict "a" 1 "b" 2))  ; 返回 ["a", "b"]

; 检查键是否存在 / Check if key exists
(dict-has (dict "name" "Evo-lang") "name")  ; 返回 true

; 字典合并 / Dictionary merge (V1.0.51)
(dict-merge (dict "a" 1 "b" 2) (dict "c" 3))  ; 返回 {"a": 1, "b": 2, "c": 3}

; 字典大小 / Dictionary size (V1.0.51)
(dict-size (dict "x" 1 "y" 2 "z" 3))  ; 返回 3
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

### 核心文档
- [架构概览](ARCHITECTURE.md) - 项目架构和模块索引
- [项目地图](PROJECT_MAP.md) - 精简版代码导航
- [API参考](docs/API-REFERENCE.md) - 关键API函数签名
- [快速入门指南](docs/getting-started.md) - 学习如何使用 Evo-lang
- [语法参考](docs/syntax-reference.md) - 完整的语法文档
- [高级特性](docs/advanced-features.md) - 深入了解语言特性

### 参考文档
- [功能清单](docs/FEATURES.md) - 完整的已实现功能列表
- [变更日志](CHANGELOG.md) - 开发历史和变更记录
- [文档索引](docs/README.md) - 完整的文档导航
- [示例代码](examples/) - 各种示例程序

## AI Agent 快速导航 / AI Agent Quick Navigation

> 💡 **提示**: 对于大型代码库，优先查阅索引文档可大幅减少token使用

### 📚 推荐阅读顺序

1. **项目概览** → `README.md` 的项目结构部分（第7-40行）
2. **架构理解** → `ARCHITECTURE.md` - 完整的模块索引和数据流图
3. **API查找** → `docs/API-REFERENCE.md` - 快速查找函数签名
4. **代码定位** → `PROJECT_MAP.md` - 精简文件映射表

### 🎯 按任务快速定位

#### 想了解解析流程？
- **入口**: `ARCHITECTURE.md` → 解析层部分
- **代码**: `src/parser/adaptive.rs` - `AdaptiveParser::parse()`
- **相关**: `src/grammar/core.rs` - AST定义

#### 想了解执行流程？
- **入口**: `ARCHITECTURE.md` → 运行时部分
- **代码**: `src/runtime/interpreter.rs` - `Interpreter::execute()`
- **相关**: `docs/API-REFERENCE.md` - Runtime API部分

#### 想了解进化机制？
- **入口**: `ARCHITECTURE.md` → 进化引擎部分
- **代码**: `src/evolution/engine.rs` - `EvolutionEngine::start_evolution()`
- **相关**: `src/evolution/knowledge.rs`, `src/evolution/tracker.rs`

#### 想了解自然语言理解？
- **代码**: `src/parser/nlu.rs` - `NLU::parse_intent()`
- **示例**: `examples/` 目录中的示例文件

#### 想了解JIT优化？
- **代码**: `src/runtime/jit.rs` - `JITCompiler::compile()`
- **示例**: `examples/jit_test.evo`

#### 想了解Python集成？
- **代码**: `src/lib.rs` - PyO3导出，`src/python/bridge.rs` - 桥接实现
- **文档**: `python/README.md`

#### 想查找某个功能的实现？
- **步骤1**: 查看 `PROJECT_MAP.md` 的快速查找表
- **步骤2**: 查看 `ARCHITECTURE.md` 的模块索引
- **步骤3**: 查看 `docs/API-REFERENCE.md` 的API签名

#### 想理解模块依赖关系？
- **文档**: `ARCHITECTURE.md` → 模块依赖关系部分
- **代码**: 各模块的 `mod.rs` 文件

### 🔍 高效搜索策略

1. **从索引开始**: 先看 `ARCHITECTURE.md` 或 `PROJECT_MAP.md`，不要直接搜索代码
2. **使用API参考**: 在 `docs/API-REFERENCE.md` 查找函数签名，避免阅读完整实现
3. **查看模块注释**: 各 `mod.rs` 文件包含模块级导航信息
4. **定位关键方法**: 根据架构图定位到具体方法，而不是整文件读取

### 📖 相关资源

- **配置文件**: `.cursorrules` - Cursor IDE 的代码风格和导航规则
- **文档目录**: `docs/` - 包含详细的使用文档
- **示例代码**: `examples/` - 包含各种功能的示例

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

当前版本：**v1.0** (持续开发中)

### 核心功能 ✅

- ✅ **完整的解析器和解释器** - 支持 S-expression 语法解析和执行
- ✅ **自然语言理解（NLU）** - 基于规则的意图识别，支持中英文输入
- ✅ **进化引擎** - 15个分析工具模块，支持自我进化和学习
- ✅ **JIT编译器** - 热点代码检测和优化
- ✅ **Python互操作** - 完整的PyO3集成
- ✅ **模块系统** - 模块导入和命名空间支持
- ✅ **数据结构** - 列表、字典及丰富的操作函数
- ✅ **标准库** - 用Evo-lang实现的核心工具函数

### 详细功能列表

查看 [完整功能清单](docs/FEATURES.md) 了解所有已实现功能的详细信息。

### 开发历史

查看 [变更日志](CHANGELOG.md) 了解详细的开发历程和变更记录。

## 贡献 / Contributing

欢迎贡献代码！请查看 [贡献指南](CONTRIBUTING.md) 了解如何参与项目。

欢迎多种贡献方式：
- 💻 **代码贡献** - 编写代码提交 PR
- 🖥️ **算力贡献** - 运行本地进化引擎，提交进化事件
- 📊 **使用贡献** - 正常使用即可，系统自动收集使用数据
- 🎯 **决策贡献** - 参与项目治理和决策

详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

Welcome to contribute! See [Contributing Guide](CONTRIBUTING.md) for how to participate.

Multiple contribution methods are welcome:
- 💻 **Code Contribution** - Write code and submit PRs
- 🖥️ **Compute Contribution** - Run local evolution engine, submit evolution events
- 📊 **Usage Contribution** - Just use the language, system automatically collects data
- 🎯 **Governance Contribution** - Participate in project governance

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 许可证 / License

本项目采用 Apache License 2.0 许可证。

This project is licensed under the Apache License 2.0.

详见 [LICENSE](LICENSE) 文件。

See the [LICENSE](LICENSE) file for details.

