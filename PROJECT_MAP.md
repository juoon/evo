# 项目地图 / Project Map

精简版项目导航索引，帮助快速定位代码位置。

## 核心路径 / Core Paths

```
输入 → grammar/ → parser/ → runtime/ → 输出
                ↓
            evolution/ (自我改进)
```

## 关键文件映射 / Key Files Map

### 语法定义 / Grammar
```
grammar/core.rs         → AST节点、数据类型定义
grammar/rule.rs         → 语法规则系统
grammar/self_desc.rs    → 自描述语法
```

### 解析 / Parsing
```
parser/adaptive.rs      → 主解析器（入口）
parser/nlu.rs           → 自然语言理解
parser/context.rs       → 上下文管理
parser/explainer.rs     → 代码解释
```

### 运行时 / Runtime
```
runtime/interpreter.rs  → 解释器核心（执行）
runtime/jit.rs          → JIT编译器（优化）
runtime/jit_interpreter.rs → JIT解释器（整合）
runtime/mode.rs         → 执行模式选择
```

### 进化引擎 / Evolution (15个模块)
```
evolution/engine.rs     → 引擎核心（协调）
evolution/knowledge.rs  → 知识图谱
evolution/tracker.rs    → 历史追踪
evolution/analyzer.rs   → 代码分析
evolution/learning.rs   → 模式学习
evolution/code_generator.rs → 代码生成
evolution/optimizer.rs  → 优化建议
evolution/quality_assessor.rs → 质量评估
evolution/code_reviewer.rs → 代码审查
evolution/performance.rs → 性能分析
evolution/doc_generator.rs → 文档生成
evolution/test_generator.rs → 测试生成
evolution/similarity.rs → 相似度检测
evolution/dependency.rs → 依赖分析
evolution/error_recovery.rs → 错误恢复
```

### 其他 / Others
```
python/bridge.rs        → Python桥接
poetry/parser.rs        → 诗歌解析
poetry/emotion.rs       → 情感分析
lib.rs                  → Python模块导出
main.rs                 → 程序入口
```

## 快速查找表 / Quick Lookup Table

| 想要... | 查看文件 |
|---------|---------|
| 了解语法结构 | `grammar/core.rs` |
| 了解解析逻辑 | `parser/adaptive.rs` |
| 了解执行逻辑 | `runtime/interpreter.rs` |
| 了解自然语言 | `parser/nlu.rs` |
| 了解进化机制 | `evolution/engine.rs` |
| 了解知识图谱 | `evolution/knowledge.rs` |
| 了解JIT优化 | `runtime/jit.rs` |
| 了解代码分析 | `evolution/analyzer.rs` |
| 了解Python集成 | `lib.rs` + `python/bridge.rs` |
| 了解错误处理 | `evolution/error_recovery.rs` |

## 数据流简图 / Data Flow Diagram

```
[源代码/自然语言]
    ↓
[AdaptiveParser] (parser/adaptive.rs)
    ↓
[AST: GrammarElement[]] (grammar/core.rs)
    ↓
[Interpreter] (runtime/interpreter.rs)
    ↓
[Value] (grammar/core.rs)
```

```
[使用模式]
    ↓
[Learning] (evolution/learning.rs)
    ↓
[Knowledge Graph] (evolution/knowledge.rs)
    ↓
[Evolution Engine] (evolution/engine.rs)
    ↓
[新规则/优化]
```

## 模块大小估算 / Module Size Estimate

- `grammar/` - 小（3个文件，核心定义）
- `parser/` - 中（4个文件，解析逻辑）
- `runtime/` - 中（4个文件，执行逻辑）
- `evolution/` - 大（15个文件，功能丰富）
- `python/` - 小（2个文件，桥接层）
- `poetry/` - 小（2个文件，特殊功能）

## 依赖关系简图 / Dependency Graph

```
core.rs (基础)
  ↑
rule.rs, self_desc.rs, parser/, runtime/
  ↑
evolution/, python/
```

## 文档位置 / Documentation Locations

- 项目概览: `README.md`
- 架构详情: `ARCHITECTURE.md`
- API参考: `docs/API-REFERENCE.md`
- 语法文档: `docs/syntax-reference.md`
- 入门指南: `docs/getting-started.md`
- 高级特性: `docs/advanced-features.md`
- 示例代码: `examples/` + `examples/README.md`
