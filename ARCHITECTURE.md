# 项目架构概览 / Architecture Overview

本文档为 AI Agent 和开发者提供项目的快速导航和架构理解。

## 核心模块快速索引 / Core Modules Quick Index

### 语法层 (Grammar Layer)
**职责**: 定义语言的语法结构、AST节点类型、语法规则系统

- `src/grammar/core.rs` - **核心语法定义**
  - AST节点类型 (`GrammarElement`)
  - 数据类型 (`Value`, `Int`, `Float`, `String`, `Bool`, `List`, `Dict`)
  - 表达式类型 (变量、函数调用、字面量等)

- `src/grammar/rule.rs` - **语法规则系统**
  - 语法规则定义 (`GrammarRule`)
  - 规则匹配和应用
  - 规则优先级和冲突处理

- `src/grammar/self_desc.rs` - **自描述语法机制**
  - 用语言自身描述语法规则
  - 语法规则的元数据

### 解析层 (Parser Layer)
**职责**: 将源代码解析为AST，支持自然语言输入

- `src/parser/adaptive.rs` - **自适应解析器主逻辑**
  - 主入口: `AdaptiveParser::new()`, `parse()`
  - S-expression 解析
  - AST构建

- `src/parser/nlu.rs` - **自然语言理解**
  - 意图识别（函数定义、变量定义、操作表达式等）
  - 中英文自然语言转代码
  - 规则匹配和代码生成

- `src/parser/context.rs` - **上下文管理**
  - 多轮对话上下文维护
  - 变量和函数引用解析
  - 上下文感知的代码生成

- `src/parser/explainer.rs` - **代码解释器**
  - 将代码结构转换为自然语言
  - 中英文双语解释

### 运行时 (Runtime)
**职责**: 执行AST，支持多种执行模式（解释、编译、JIT）

- `src/runtime/interpreter.rs` - **解释器核心**
  - 主入口: `Interpreter::new()`, `execute()`
  - 表达式求值
  - 函数调用、递归、闭包
  - 作用域管理 (`let` 绑定)

- `src/runtime/jit.rs` - **JIT编译器**
  - 热点代码检测
  - 常量折叠优化
  - 代码缓存和执行优化

- `src/runtime/jit_interpreter.rs` - **JIT解释器**
  - 整合解释器和JIT编译器
  - 自动选择执行模式

- `src/runtime/mode.rs` - **执行模式选择**
  - 解释模式 vs JIT模式
  - 性能分析和模式切换

### 进化引擎 (Evolution Engine)
**职责**: 语言的自我进化、学习、优化和代码质量提升

#### 核心组件 (Core Components)
- `src/evolution/engine.rs` - **进化引擎核心**
  - 主入口: `EvolutionEngine::new()`, `start_evolution()`, `self_evolve()`
  - 进化流程协调
  - 规则生成和应用

- `src/evolution/knowledge.rs` - **知识图谱**
  - 实体提取和关系挖掘
  - 模式发现
  - 相似度计算

- `src/evolution/tracker.rs` - **进化历史追踪**
  - 进化事件记录
  - 谱系树构建
  - 回滚机制 (`rollback_to_event()`)
  - 事件保存和加载 (`save_all_events()`, `load_events_from_dir()`)

- `src/evolution/event_manager.rs` - **进化事件管理器**
  - 进化事件保存到文件 (`save_event()`)
  - 从文件加载进化事件 (`load_event()`, `load_all_events()`)
  - 进化事件验证 (`validate_event()`)
  - 进化事件冲突检测 (`detect_conflicts()`)
  - 进化事件合并 (`merge_events()`)
  - 最优事件选择（冲突时）

#### 分析工具 (Analysis Tools)
- `src/evolution/analyzer.rs` - **代码分析器**
  - 代码模式识别
  - 复杂度分析
  - 优化建议生成

- `src/evolution/learning.rs` - **使用模式学习**
  - 使用频率跟踪
  - 错误模式学习
  - 成功模式识别

- `src/evolution/similarity.rs` - **相似度检测**
  - 代码重复检测
  - 结构相似度计算
  - 重构建议

- `src/evolution/dependency.rs` - **依赖分析**
  - 依赖图构建
  - 循环依赖检测
  - 依赖统计

#### 代码生成和优化 (Code Generation & Optimization)
- `src/evolution/code_generator.rs` - **智能代码生成**
  - 基于意图生成代码
  - 代码模板库
  - 代码补全建议

- `src/evolution/optimizer.rs` - **优化建议器**
  - 优化策略库
  - 基于质量评估的优化建议
  - 优化效果预测

- `src/evolution/error_recovery.rs` - **错误恢复**
  - 常见错误自动修复
  - 智能错误建议

#### 质量评估 (Quality Assessment)
- `src/evolution/quality_assessor.rs` - **代码质量评估**
  - 多维度评估（可读性、可维护性、性能等）
  - 质量等级评分
  - 改进建议

- `src/evolution/code_reviewer.rs` - **代码审查**
  - 自动问题检测
  - 审查报告生成
  - 审查历史记录

- `src/evolution/performance.rs` - **性能分析**
  - 时间复杂度分析
  - 性能瓶颈识别
  - 性能优化建议

- `src/evolution/doc_generator.rs` - **文档生成**
  - 自动生成代码文档（Markdown、HTML等）
  - 文档质量评估

- `src/evolution/test_generator.rs` - **测试生成**
  - 自动生成测试用例
  - 测试覆盖率分析

### 其他模块 (Other Modules)

- `src/python/` - **Python互操作层**
  - PyO3 集成
  - Python API 导出
  - 类型转换

- `src/poetry/` - **诗歌理解模块**
  - 诗歌解析
  - 情感分析
  - 主题和意象提取

## 关键数据流 / Key Data Flow

### 1. 代码执行流程 (Code Execution Flow)
```
源代码 (Source Code)
  ↓
AdaptiveParser::parse() [parser/adaptive.rs]
  ↓
AST (Vec<GrammarElement>) [grammar/core.rs]
  ↓
Interpreter::execute() [runtime/interpreter.rs]
  ↓
运行时值 (Value) [grammar/core.rs]
```

### 2. 自然语言编程流程 (Natural Language Programming Flow)
```
自然语言输入
  ↓
NLU::parse_intent() [parser/nlu.rs]
  ↓
意图识别 + 代码生成
  ↓
AST (通过 AdaptiveParser)
  ↓
执行
```

### 3. 进化流程 (Evolution Flow)
```
使用模式 / 代码输入
  ↓
UsagePatternLearner [evolution/learning.rs]
  ↓
知识图谱更新 [evolution/knowledge.rs]
  ↓
进化引擎分析 [evolution/engine.rs]
  ↓
规则生成和评估
  ↓
进化事件记录 [evolution/tracker.rs]
  ↓
规则应用 (更新语法规则)
```

### 4. JIT优化流程 (JIT Optimization Flow)
```
代码执行统计 [runtime/interpreter.rs]
  ↓
热点检测 [runtime/jit.rs]
  ↓
常量折叠 + 代码优化
  ↓
编译并缓存
  ↓
优化执行
```

### 5. 自进化闭环 (Self-Evolution Loop)
```
当前规则和代码
  ↓
CodeAnalyzer [evolution/analyzer.rs]
  ↓
QualityAssessor [evolution/quality_assessor.rs]
  ↓
OptimizationAdvisor [evolution/optimizer.rs]
  ↓
进化引擎生成改进 [evolution/engine.rs]
  ↓
应用到代码/规则
```

## 模块依赖关系 / Module Dependencies

```
grammar/
  ├─ core.rs (基础定义，无依赖)
  ├─ rule.rs (依赖 core.rs)
  └─ self_desc.rs (依赖 core.rs, rule.rs)

parser/
  ├─ adaptive.rs (依赖 grammar/)
  ├─ nlu.rs (依赖 grammar/, adaptive.rs)
  ├─ context.rs (依赖 grammar/)
  └─ explainer.rs (依赖 grammar/)

runtime/
  ├─ interpreter.rs (依赖 grammar/)
  ├─ jit.rs (依赖 grammar/, interpreter.rs)
  ├─ jit_interpreter.rs (依赖 interpreter.rs, jit.rs)
  └─ mode.rs (依赖 interpreter.rs, jit.rs)

evolution/
  ├─ engine.rs (核心，依赖多个子模块)
  ├─ knowledge.rs (依赖 grammar/)
  ├─ tracker.rs (基础模块)
  └─ [其他工具模块] (依赖 grammar/, 部分依赖 engine.rs)

python/
  └─ bridge.rs (依赖 parser/, runtime/)

poetry/
  └─ parser.rs, emotion.rs (相对独立，可集成到 evolution/)
```

## 快速查找指南 / Quick Lookup Guide

### 想了解语法定义？
→ `src/grammar/core.rs` - 查看 `GrammarElement` 和 `Value` 类型定义

### 想了解解析逻辑？
→ `src/parser/adaptive.rs` - 查看 `AdaptiveParser::parse()` 方法

### 想了解执行逻辑？
→ `src/runtime/interpreter.rs` - 查看 `Interpreter::execute()` 方法

### 想了解自然语言理解？
→ `src/parser/nlu.rs` - 查看 `NLU::parse_intent()` 方法

### 想了解进化机制？
→ `src/evolution/engine.rs` - 查看 `EvolutionEngine::start_evolution()` 和 `self_evolve()` 方法

### 想了解进化事件管理？
→ `src/evolution/event_manager.rs` - 查看 `EvolutionEventManager`（保存、加载、合并事件）

### 想了解知识图谱？
→ `src/evolution/knowledge.rs` - 查看 `KnowledgeGraph` 结构

### 想了解JIT优化？
→ `src/runtime/jit.rs` - 查看 `JITCompiler::compile()` 方法

### 想了解代码分析？
→ `src/evolution/analyzer.rs` - 查看 `CodeAnalyzer` 结构

### 想了解Python集成？
→ `src/python/bridge.rs` 和 `src/lib.rs` - 查看 PyO3 导出

## 设计原则 / Design Principles

1. **分层架构** - 语法层 → 解析层 → 运行时层 → 进化层
2. **模块化设计** - 每个模块职责单一，接口清晰
3. **自举能力** - 语言能够用自身描述和优化自身
4. **渐进式扩展** - 从简单核心开始，逐步添加功能
5. **可追溯性** - 所有进化过程都被记录和追踪

## 扩展点 / Extension Points

- **新语法规则**: 在 `grammar/rule.rs` 中添加规则定义
- **新解析特性**: 在 `parser/` 中添加新的解析逻辑
- **新运行时特性**: 在 `runtime/` 中添加新的执行模式
- **新分析工具**: 在 `evolution/` 中添加新的分析模块
- **新数据类型**: 在 `grammar/core.rs` 中的 `Value` 枚举添加变体
