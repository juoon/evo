# 贡献指南 / Contributing Guide

感谢您对 Evo-lang 项目的关注！本文档将指导您如何为这个自进化编程语言项目做出贡献。

Thank you for your interest in the Evo-lang project! This guide will help you contribute to this self-evolving programming language project.

## 目录 / Table of Contents

- [贡献方式 / Contribution Methods](#贡献方式--contribution-methods)
- [代码贡献 / Code Contribution](#代码贡献--code-contribution)
- [算力贡献 / Compute Contribution](#算力贡献--compute-contribution)
- [使用贡献 / Usage Contribution](#使用贡献--usage-contribution)
- [决策贡献 / Governance Contribution](#决策贡献--governance-contribution)
- [项目目标和约束 / Project Goals and Constraints](#项目目标和约束--project-goals-and-constraints)
- [代码规范 / Code Style](#代码规范--code-style)
- [提交规范 / Commit Guidelines](#提交规范--commit-guidelines)
- [许可证 / License](#许可证--license)

## 贡献方式 / Contribution Methods

Evo-lang 作为一门自进化编程语言，支持多种贡献方式。您可以选择最适合您的方式参与项目。

Evo-lang, as a self-evolving programming language, supports multiple contribution methods. You can choose the method that best suits you.

### 贡献方式对比 / Contribution Methods Comparison

| 方式 | 名称 | 难度 | 时间投入 | 是否需要提交 |
|------|------|------|----------|--------------|
| 方式1 | 算力贡献 | 低 | 灵活 | 是（进化事件） |
| 方式2 | 使用贡献 | 极低 | 无额外投入 | 否（自动收集） |
| 方式3 | 代码贡献 | 中高 | 较多 | 是（代码PR） |
| 方式4 | 决策贡献 | 高 | 长期 | 是（文档/决策） |

## 代码贡献 / Code Contribution

### 传统方式：编写代码提交 PR

这是最直接的贡献方式，适合有 Rust 开发经验的贡献者。

#### 准备工作

1. **Fork 仓库**
   ```bash
   git clone https://github.com/your-username/evo.git
   cd evo
   ```

2. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **熟悉项目结构**
   - 查阅 [ARCHITECTURE.md](ARCHITECTURE.md) 了解项目架构
   - 查阅 [PROJECT_MAP.md](PROJECT_MAP.md) 快速定位代码
   - 查阅 [docs/API-REFERENCE.md](docs/API-REFERENCE.md) 了解API接口

#### 开发流程

1. **遵循代码规范**
   - 使用 `rustfmt` 格式化代码
   - 参考 `.cursorrules` 中的代码风格指南
   - 为关键函数添加文档注释（使用 `///`）

2. **编写测试**
   - 为新功能添加测试用例
   - 确保所有测试通过：`cargo test`

3. **更新文档**
   每次提交前必须更新相关文档：
   - ✅ **CHANGELOG.md** - 添加新的任务记录
   - ✅ **docs/FEATURES.md** - 如果是新功能
   - ✅ **ARCHITECTURE.md** - 如果改变架构
   - ✅ **docs/API-REFERENCE.md** - 如果有新API

4. **提交代码**
   ```bash
   # 格式化代码
   cargo fmt
   
   # 运行测试
   cargo test
   
   # 提交（带上版本号）
   git commit -m "V1.0.XX: 功能描述 / Feature description"
   git push origin feature/your-feature-name
   ```

5. **创建 Pull Request**
   - 在 GitHub 上创建 PR
   - 填写清晰的 PR 描述
   - 等待代码审查

### AI辅助方式：使用 prompt.txt 辅助开发

如果您是新手或希望快速实现功能，可以使用项目的 `prompt.txt` 来指导 AI 助手（如 Cursor）自动生成代码。

1. **打开 prompt.txt**
   - 该文件包含了项目目标、工作流程和代码规范

2. **让 AI 助手根据 prompt.txt 生成代码**
   - AI 会自动遵循项目规范
   - 自动更新相关文档

3. **审查生成的代码**
   - 确保代码质量
   - 运行测试验证

4. **提交代码**（同传统方式）

## 算力贡献 / Compute Contribution

### 什么是算力贡献？

算力贡献是指运行本地进化引擎，让代码自动进化，然后提交进化事件（而非直接代码）。

**优势**：
- ✅ 无需编写代码，只需提供计算资源
- ✅ 系统自动根据 `prompt.txt` 进行进化
- ✅ 提交的是结构化的进化事件，易于追踪和验证

### 算力贡献工作流程

#### 1. 设置本地环境

```bash
# 克隆仓库
git clone https://github.com/juoon/evo.git
cd evo

# 编译项目
cargo build --release
```

#### 2. 启动本地进化引擎

```bash
# 运行进化模式（使用 prompt.txt 作为指导）
cargo run -- --evolution-mode --prompt prompt.txt
```

系统会自动：
- 读取 `prompt.txt` 了解项目目标
- 分析当前代码状态
- 基于目标自动生成进化方案
- 生成进化事件文件

#### 3. 提交进化事件

系统会自动生成进化事件文件到 `evolution_events/` 目录：

```bash
# 查看生成的进化事件
ls evolution_events/
# event_abc123.json
# event_def456.json

# 提交进化事件（而非代码）
git add evolution_events/event_*.json
git commit -m "Compute contribution: auto-evolution events from local computation"
git push origin feature/compute-contribution
```

#### 4. 进化事件格式

每个进化事件包含：

```json
{
  "id": "uuid",
  "timestamp": "2024-01-15T10:30:00Z",
  "event_type": "SyntaxEvolution",
  "base_version": "v1.0.56,
  "delta": {
    "added_rules": [...],      // 新增的语法规则
    "modified_rules": [],      // 修改的规则
    "removed_rules": [],       // 删除的规则
    "description": "描述"
  },
  "trigger": {
    "source": "AutomaticOptimization",
    "conditions": ["prompt.txt指导", "使用模式分析"]
  },
  "success_metrics": {
    "performance_improvement": 0.05,
    "compatibility_impact": 0.0
  }
}
```

### 服务器端合并

当进化事件被提交到仓库后：

1. **自动验证**
   - 检查事件格式
   - 验证语法规则有效性
   - 运行测试

2. **冲突检测**
   - 检测是否与其他进化事件冲突
   - 使用知识图谱分析影响范围

3. **自动合并**
   - 如果兼容，自动合并
   - 如果不兼容，选择最优事件（基于指标）

4. **应用到代码库**
   - 自动生成代码变更
   - 更新相关文件

## 使用贡献 / Usage Contribution

### 什么是使用贡献？

使用贡献是最简单的贡献方式：**正常使用 Evo-lang 即可**，系统会自动收集使用模式数据。

**无需任何主动操作！**

### 使用贡献如何帮助进化？

#### 自动数据收集

每次使用语言时，系统自动记录：

- **使用频率**：哪些语法特性最常用
- **错误模式**：哪些错误最常见
- **成功模式**：哪些代码模式最成功
- **性能数据**：执行时间和内存使用

#### 自动学习与进化

当数据积累到一定程度时，系统会：

1. **自动分析**
   - 识别常见错误模式
   - 发现高频使用模式
   - 检测性能瓶颈

2. **生成洞察**
   - 自动生成改进建议
   - 识别优化机会

3. **触发进化**
   - 高优先级洞察自动触发进化事件
   - 本地应用改进（如适用）

### 使用贡献示例

```rust
// 您正常使用 Evo-lang
let code = "(let x 5)";
interpreter.execute(&code); // ← 系统自动记录：变量定义模式

// 遇到错误
let code = "(let y (+ x 1))"; // x未定义
// ← 系统自动记录：UndefinedVariable错误

// 使用成功
let code = "(def add (x y) (+ x y))";
// ← 系统自动记录：函数定义成功模式
```

所有这些都会被自动记录和分析，无需您做任何额外工作！

## 决策贡献 / Governance Contribution

### 什么是决策贡献？

决策贡献是指参与项目治理、设定目标和约束、审查进化结果。

适合有经验的开发者或项目维护者。

### 决策贡献方式

#### 1. 设定项目目标

在 `prompt.txt` 中设定：

```markdown
## 项目目标 / Project Goals

### 终极目标
理解人类思想，促进人类与智能生命和谐共生

### 当前重点
- 自进化能力的持续完善
- 理解能力的深度增强
- ...
```

#### 2. 设定约束

通过配置文件设定约束：

```toml
# evolution_config.toml

[constraints]
hard_constraints = [
    "保持向后兼容性",
    "不破坏现有API"
]

performance_constraints = [
    "性能不得下降超过10%",
    "内存使用不得超过当前120%"
]
```

#### 3. 审查进化结果

- 审查自动进化的结果
- 决定哪些进化事件应该合并
- 设定进化优先级

#### 4. 参与讨论

- 在 Issues 中讨论项目方向
- 参与设计决策
- 分享经验和建议

## 项目目标和约束 / Project Goals and Constraints

### 终极目标

**理解人类思想，促进人类与智能生命和谐共生**

这是项目的长期愿景，所有进化都应该朝着这个方向努力。

### 当前重点

- 自进化能力的持续完善
- 理解能力的深度增强
- 代码生成与优化的智能化
- 自举能力的扩展
- 记忆与反思机制的改进

查看 `docs/evolution-status.md` 了解详细进展。

### 约束条件

所有贡献和进化都需要遵守以下约束：

1. **兼容性约束**
   - 保持向后兼容性
   - 不破坏现有 API
   - 渐进式改进而非激进变革

2. **性能约束**
   - 性能不得下降超过 10%
   - 内存使用需控制在合理范围

3. **质量约束**
   - 代码复杂度需在合理范围
   - 测试覆盖率需达标

4. **架构约束**
   - 遵循模块化设计原则
   - 保持代码清晰和可维护

## 代码规范 / Code Style

### Rust 代码

- 遵循标准 `rustfmt` 风格
- 关键函数必须有文档注释（使用 `///`）
- 模块级文档注释（使用 `//!`）应包含：
  - 模块职责说明
  - 快速导航信息
  - 关键数据结构和方法

### 文档注释格式

```rust
/// Function description
/// 
/// # Arguments
/// * `arg` - Argument description
/// 
/// # Returns
/// Return value description
pub fn function(arg: Type) -> ReturnType {
    // ...
}
```

### 命名规范

- 函数名：`snake_case`
- 类型名：`PascalCase`
- 常量名：`UPPER_SNAKE_CASE`
- 模块名：`snake_case`

详细规范请参考 `.cursorrules`。

## 提交规范 / Commit Guidelines

### 提交消息格式

```
V版本号: 简短描述 / Short description

详细说明（可选）/ Detailed description (optional)
```

示例：

```
V1.0.56 添加列表推导式语法支持 / Add list comprehension syntax support

- 新增语法规则：list_comprehension
- 更新解析器以支持新语法
- 添加相关测试用例
- 更新文档和API参考
```

### 提交前检查清单

- [ ] 代码已格式化（`cargo fmt`）
- [ ] 测试已通过（`cargo test`）
- [ ] CHANGELOG.md 已更新
- [ ] 如果是新功能，FEATURES.md 已更新
- [ ] 如果改变架构，ARCHITECTURE.md 已更新
- [ ] 如果有新API，API-REFERENCE.md 已更新

## 进化事件合并 / Evolution Event Merging

### 合并策略

多个进化事件的合并遵循以下策略：

1. **基于版本的事件排序**
   - 每个事件包含 `base_version`
   - 按依赖关系排序

2. **兼容性检查**
   - 检查是否修改相同的规则/文件
   - 检查是否有语义冲突
   - 检查是否有性能回归

3. **基于指标的选择**
   - 如果不兼容，选择更优的事件（基于 `success_metrics`）
   - 考虑性能、兼容性、用户满意度等因素

### 冲突解决

如果两个进化事件存在冲突：

1. **自动检测冲突**
   - 使用知识图谱分析影响范围
   - 使用相似度分析判断是否冲突

2. **自动选择策略**
   - 选择指标更好的事件
   - 或要求贡献者协调

3. **人工介入**（如需要）
   - 维护者可以手动干预
   - 合并双方的优势

## 许可证 / License

### 贡献者协议

通过向本项目贡献代码，您同意您的贡献将在 Apache License 2.0 许可证下授权。

By contributing to this project, you agree that your contributions will be licensed under the Apache License 2.0.

### 版权

贡献者的版权归贡献者所有。无需版权转让。

Contributors retain copyright to their contributions. No copyright assignment is required.

### 详细条款

详见 [LICENSE](LICENSE) 文件。

See the [LICENSE](LICENSE) file for details.

## 获取帮助 / Getting Help

### 问题反馈

- 创建 [Issue](https://github.com/juoon/evo/issues) 报告问题
- 在 Discussions 中讨论想法

### 文档资源

- [README.md](README.md) - 项目概览
- [ARCHITECTURE.md](ARCHITECTURE.md) - 架构文档
- [docs/API-REFERENCE.md](docs/API-REFERENCE.md) - API 参考
- [docs/getting-started.md](docs/getting-started.md) - 快速入门

### 社区

- 欢迎参与讨论
- 分享使用经验
- 提出改进建议

## 致谢 / Acknowledgments

感谢所有为 Evo-lang 做出贡献的开发者！

Thank you to all contributors who help make Evo-lang better!

---

**让我们一起推动编程语言的进化！** 🚀

**Let's evolve programming languages together!** 🚀
