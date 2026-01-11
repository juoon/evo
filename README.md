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
│   └── mode.rs          # 执行模式选择 / Execution mode selection
├── python/              # Python兼容层 / Python compatibility layer
│   ├── mod.rs
│   └── bridge.rs        # Python桥接 / Python bridge
└── poetry/              # 诗歌理解模块 / Poetry understanding module
    ├── mod.rs
    ├── parser.rs        # 诗歌解析 / Poetry parser
    └── emotion.rs       # 情感理解 / Emotion understanding
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

### 5. 诗歌理解能力 / Poetry Understanding Capability
- 以《静夜思》为起点的情感理解 / Emotion understanding starting with "Quiet Night Thoughts"
- 主题和意象提取 / Theme and imagery extraction
- 情感分析 / Emotion analysis

## 使用示例 / Usage Example

运行示例程序：
```bash
cargo run
```

程序将演示：
1. 《静夜思》的解析和理解 / Parsing and understanding of "Quiet Night Thoughts"
2. 语法定义示例 / Grammar definition examples
3. 进化引擎演示 / Evolution engine demonstration

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
- ✅ NLU系统框架（接口已定义，需要模型集成）
- ✅ 进化引擎核心功能
- ✅ 执行引擎（解释器）基础实现
- ✅ Python兼容层框架（需要PyO3集成）

## 下一步 / Next Steps

1. 集成NLU模型（本地轻量模型或云API）
2. 实现完整的解释器功能
3. 集成PyO3实现Python互操作
4. 实现JIT编译器
5. 完善进化引擎的学习和预测功能

## 许可证 / License

待定 / To be determined

