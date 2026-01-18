# API 快速参考 / API Quick Reference

本文档提供关键API的函数签名，帮助快速理解接口，无需深入实现代码。

## Parser / 解析器

### AdaptiveParser

```rust
// 创建自适应解析器
impl AdaptiveParser {
    pub fn new(enable_nlu: bool) -> Self;
    pub fn parse(&self, code: &str) -> Result<Vec<GrammarElement>, ParseError>;
}
```

**关键方法**:
- `new()` - 创建解析器，`enable_nlu` 控制是否启用自然语言理解
- `parse()` - 解析源代码，返回AST (`Vec<GrammarElement>`)

### NLU (Natural Language Understanding)

```rust
// 自然语言理解
impl NLU {
    pub fn parse_intent(&self, input: &str) -> Result<Intent, NLUError>;
    pub fn generate_code(&self, intent: &Intent) -> Result<String, NLUError>;
}
```

**关键方法**:
- `parse_intent()` - 识别自然语言输入的意图
- `generate_code()` - 将意图转换为Evo-lang代码

### ContextManager

```rust
// 上下文管理
impl ContextManager {
    pub fn new() -> Self;
    pub fn add_context(&mut self, context: &str);
    pub fn resolve_reference(&self, name: &str) -> Option<Value>;
    pub fn generate_with_context(&mut self, input: &str) -> Result<String>;
}
```

## Runtime / 运行时

### Interpreter

```rust
// 解释器
impl Interpreter {
    pub fn new() -> Self;
    pub fn execute(&mut self, ast: &[GrammarElement]) -> Result<Value, RuntimeError>;
    pub fn get_env(&self) -> &Environment;
    pub fn set_env(&mut self, env: Environment);
}
```

**关键方法**:
- `new()` - 创建解释器实例
- `execute()` - 执行AST，返回计算结果 (`Value`)

### JITCompiler

```rust
// JIT编译器
impl JITCompiler {
    pub fn new(threshold: usize) -> Self;
    pub fn should_compile(&self, stats: &ExecutionStats) -> bool;
    pub fn compile(&mut self, ast: &[GrammarElement]) -> Result<CompiledCode, JITError>;
    pub fn get_stats(&self) -> JITStats;
}
```

**关键方法**:
- `should_compile()` - 判断是否应该编译（基于执行统计）
- `compile()` - 编译AST为优化代码
- `get_stats()` - 获取JIT统计信息

### JITInterpreter

```rust
// JIT解释器（整合解释器和JIT）
impl JITInterpreter {
    pub fn new() -> Self;
    pub fn execute(&mut self, ast: &[GrammarElement]) -> Result<Value, RuntimeError>;
    pub fn get_jit_stats(&self) -> JITStats;
}
```

## Evolution Engine / 进化引擎

### EvolutionEngine

```rust
// 进化引擎核心
impl EvolutionEngine {
    pub fn new() -> Self;
    pub fn self_evolve(&mut self) -> Result<serde_json::Value, EvolutionError>;
    pub fn save_events_to_dir(&self, events_dir: impl AsRef<Path>) -> Result<(), EvolutionError>;
    pub fn load_events_from_dir(&mut self, events_dir: impl AsRef<Path>) -> Result<(), EvolutionError>;
    pub fn get_history(&self) -> &[EvolutionEvent];
    pub fn get_knowledge_stats(&self) -> serde_json::Value;
}
```

**关键方法**:
- `self_evolve()` - 自动分析和改进自身实现
- `save_events_to_dir()` - 保存所有进化事件到目录（用于算力贡献）
- `load_events_from_dir()` - 从目录加载进化事件
- `get_history()` - 获取进化历史
- `get_knowledge_stats()` - 获取知识图谱统计

### KnowledgeGraph

```rust
// 知识图谱
impl KnowledgeGraph {
    pub fn new() -> Self;
    pub fn add_entity(&mut self, entity: Entity);
    pub fn add_relation(&mut self, relation: Relation);
    pub fn find_similar(&self, entity: &Entity) -> Vec<Entity>;
    pub fn extract_from_code(&mut self, code: &[GrammarElement]);
}
```

### EvolutionTracker

```rust
// 进化历史追踪
impl EvolutionTracker {
    pub fn new() -> Self;
    pub fn record(&mut self, event: EvolutionEvent);
    pub fn get_history(&self) -> &[EvolutionEvent];
    pub fn save_all_events(&self, events_dir: impl AsRef<Path>) -> Result<(), String>;
    pub fn load_events_from_dir(&mut self, events_dir: impl AsRef<Path>) -> Result<(), String>;
    pub fn rollback_to(&mut self, event_id: Uuid) -> Result<StateSnapshot, String>;
}
```

**关键方法**:
- `record()` - 记录进化事件
- `get_history()` - 获取进化历史
- `save_all_events()` - 保存所有事件到目录
- `load_events_from_dir()` - 从目录加载所有事件
- `rollback_to()` - 回滚到指定事件

### EvolutionEventManager

```rust
// 进化事件管理器
impl EvolutionEventManager {
    pub fn new(events_dir: impl AsRef<Path>) -> Self;
    pub fn save_event(&self, event: &EvolutionEvent) -> Result<PathBuf, EventManagerError>;
    pub fn load_event(&self, event_id: Uuid) -> Result<EvolutionEvent, EventManagerError>;
    pub fn load_all_events(&self) -> Result<Vec<EvolutionEvent>, EventManagerError>;
    pub fn validate_event(&self, event: &EvolutionEvent) -> Result<(), EventValidationError>;
    pub fn detect_conflicts(&self, events: &[EvolutionEvent]) -> Vec<EventConflict>;
    pub fn merge_events(&self, events: Vec<EvolutionEvent>) -> Result<EvolutionEvent, EventManagerError>;
}
```

**关键方法**:
- `save_event()` - 保存单个进化事件到JSON文件
- `load_event()` - 从JSON文件加载单个进化事件
- `load_all_events()` - 从目录加载所有进化事件
- `validate_event()` - 验证进化事件的有效性
- `detect_conflicts()` - 检测多个进化事件之间的冲突
- `merge_events()` - 合并多个兼容的进化事件
- `get_lineage()` - 获取进化谱系链

## Code Analysis / 代码分析

### CodeAnalyzer

```rust
// 代码分析器
impl CodeAnalyzer {
    pub fn new() -> Self;
    pub fn analyze(&self, ast: &[GrammarElement]) -> CodeAnalysis;
    pub fn suggest_optimizations(&self, analysis: &CodeAnalysis) -> Vec<OptimizationSuggestion>;
}
```

### QualityAssessor

```rust
// 代码质量评估
impl QualityAssessor {
    pub fn new() -> Self;
    pub fn assess(&self, ast: &[GrammarElement]) -> QualityReport;
    pub fn get_improvement_suggestions(&self, report: &QualityReport) -> Vec<Suggestion>;
}
```

### CodeGenerator

```rust
// 智能代码生成
impl IntelligentCodeGenerator {
    pub fn new() -> Self;
    pub fn generate_from_intent(&self, intent: &Intent) -> Result<String, GenerationError>;
    pub fn suggest_completion(&self, partial_code: &str, context: &Context) -> Vec<String>;
}
```

## 常用类型 / Common Types

### Value (运行时值)

```rust
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Lambda {
        params: Vec<String>,
        body: Vec<GrammarElement>,
        closure: Environment,
    },
}
```

### GrammarElement (AST节点)

```rust
pub enum GrammarElement {
    Atom(Atom),
    List(Vec<GrammarElement>),
}

pub enum Atom {
    Identifier(String),
    Number(Number),
    String(String),
    Boolean(bool),
    Null,
}
```

### EvolutionEvent (进化事件)

```rust
pub struct EvolutionEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EvolutionEventType,
    pub parent_id: Option<String>,
    pub description: String,
    pub rules_added: Vec<GrammarRule>,
    pub rules_modified: Vec<GrammarRule>,
}
```

## Python API / Python接口

### EvoInterpreter (Python类)

```python
class EvoInterpreter:
    def __init__(self)
    def execute(self, code: str) -> str
    def eval(self, code: str) -> Any  # 返回Python对象
```

### EvoParser (Python类)

```python
class EvoParser:
    def __init__(self, enable_nlu: bool = True)
    def parse(self, code: str) -> dict  # 返回AST字典
```

### 全局函数

```python
def parse(code: str) -> dict
def execute(code: str) -> str
def eval(code: str) -> Any
```

## 错误类型 / Error Types

- `ParseError` - 解析错误
- `RuntimeError` - 运行时错误
- `NLUError` - 自然语言理解错误
- `EvolutionError` - 进化引擎错误
- `JITError` - JIT编译错误

## 使用示例 / Usage Examples

### 基本解析和执行

```rust
use evo::parser::AdaptiveParser;
use evo::runtime::Interpreter;

let parser = AdaptiveParser::new(true);
let mut interpreter = Interpreter::new();

let code = "(+ 1 2)";
let ast = parser.parse(code)?;
let result = interpreter.execute(&ast)?;
```

### 进化引擎使用

```rust
use evo::evolution::EvolutionEngine;

let mut engine = EvolutionEngine::new();
let event = engine.start_evolution("优化列表操作性能")?;
let self_improvement = engine.self_evolve()?;
```

### Python使用

```python
import evo

# 简单执行
result = evo.execute("(+ 1 2)")

# 使用解释器类
interpreter = evo.EvoInterpreter()
result = interpreter.execute("(def add (x y) (+ x y))")
value = interpreter.eval("(add 3 4)")  # 返回 7
```
