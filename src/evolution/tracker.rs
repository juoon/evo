// 进化记录器 / Evolution tracker
// 记录所有语法和语义的进化历史
// Records the evolutionary history of all syntax and semantics

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::grammar::rule::GrammarRule;

/// 进化记录器 / Evolution tracker
pub struct EvolutionTracker {
    /// 事件日志 / Event log
    event_log: Vec<EvolutionEvent>,
    /// 进化谱系 / Evolution genealogy
    genealogy: EvolutionGenealogy,
}

impl EvolutionTracker {
    /// 创建新追踪器 / Create new tracker
    pub fn new() -> Self {
        Self {
            event_log: Vec::new(),
            genealogy: EvolutionGenealogy::new(),
        }
    }

    /// 记录进化事件 / Record evolution event
    pub fn record(&mut self, event: EvolutionEvent) {
        // 构建进化谱系 / Build evolution genealogy
        let parents = self.find_parent_events(&event);
        self.genealogy.add_lineage(&event, parents);
        
        // 添加到事件日志 / Add to event log
        self.event_log.push(event);
    }

    /// 查找父事件 / Find parent events
    fn find_parent_events(&self, event: &EvolutionEvent) -> Vec<Uuid> {
        // TODO: 实现父事件查找逻辑 / Implement parent event finding logic
        Vec::new()
    }

    /// 获取进化历史 / Get evolution history
    pub fn get_history(&self) -> &[EvolutionEvent] {
        &self.event_log
    }

    /// 获取进化谱系 / Get evolution genealogy
    pub fn get_genealogy(&self) -> &EvolutionGenealogy {
        &self.genealogy
    }
}

impl Default for EvolutionTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// 进化事件 / Evolution event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEvent {
    /// 事件ID / Event ID
    pub id: Uuid,
    /// 时间戳 / Timestamp
    pub timestamp: DateTime<Utc>,
    /// 事件类型 / Event type
    pub event_type: EvolutionType,
    /// 变化前的状态 / State before change
    pub before_state: StateSnapshot,
    /// 变化后的状态 / State after change
    pub after_state: StateSnapshot,
    /// 变化内容 / Change content
    pub delta: EvolutionDelta,
    /// 触发上下文 / Trigger context
    pub trigger: TriggerContext,
    /// 作者 / Author (None表示自动进化 / None means automatic evolution)
    pub author: Option<String>,
    /// 成功指标 / Success metrics
    pub success_metrics: Option<EvolutionMetrics>,
}

/// 进化类型 / Evolution type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionType {
    /// 语法进化 / Syntax evolution
    SyntaxEvolution,
    /// 语义进化 / Semantic evolution
    SemanticEvolution,
    /// 性能进化 / Performance evolution
    PerformanceEvolution,
    /// 生态进化 / Ecosystem evolution
    EcosystemEvolution,
    /// 交互进化 / Interaction evolution
    InteractionEvolution,
}

/// 状态快照 / State snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    /// 语法规则列表 / Grammar rules list
    pub grammar_rules: Vec<GrammarRule>,
    /// 版本号 / Version number
    pub version: String,
    /// 其他状态数据 / Other state data
    pub metadata: serde_json::Value,
}

/// 进化变化 / Evolution delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionDelta {
    /// 添加的规则 / Added rules
    pub added_rules: Vec<GrammarRule>,
    /// 修改的规则 / Modified rules
    pub modified_rules: Vec<(GrammarRule, GrammarRule)>,
    /// 删除的规则 / Removed rules
    pub removed_rules: Vec<GrammarRule>,
    /// 变化描述 / Change description
    pub description: String,
}

/// 触发上下文 / Trigger context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerContext {
    /// 触发源 / Trigger source
    pub source: TriggerSource,
    /// 触发条件 / Trigger conditions
    pub conditions: Vec<String>,
    /// 环境信息 / Environment information
    pub environment: serde_json::Value,
}

/// 触发源 / Trigger source
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerSource {
    /// 用户显式请求 / User explicit request
    UserRequest,
    /// 自然语言指令 / Natural language instruction
    NaturalLanguageInstruction,
    /// 使用模式分析 / Usage pattern analysis
    UsagePatternAnalysis,
    /// 性能监控 / Performance monitoring
    PerformanceMonitoring,
    /// 社区投票 / Community voting
    CommunityVoting,
    /// 自动优化 / Automatic optimization
    AutomaticOptimization,
}

/// 进化指标 / Evolution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionMetrics {
    /// 进化成功率 / Evolution success rate
    pub success_rate: f64,
    /// 用户满意度变化 / User satisfaction change
    pub user_satisfaction_delta: f64,
    /// 性能提升幅度 / Performance improvement
    pub performance_improvement: f64,
    /// 兼容性影响 / Compatibility impact
    pub compatibility_impact: f64,
}

/// 进化谱系 / Evolution genealogy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionGenealogy {
    /// 谱系图 / Genealogy graph (parent -> children mapping)
    pub lineages: std::collections::HashMap<Uuid, Vec<Uuid>>,
}

impl EvolutionGenealogy {
    /// 创建新谱系 / Create new genealogy
    pub fn new() -> Self {
        Self {
            lineages: std::collections::HashMap::new(),
        }
    }

    /// 添加谱系关系 / Add lineage relationship
    pub fn add_lineage(&mut self, event: &EvolutionEvent, parents: Vec<Uuid>) {
        for parent_id in parents {
            self.lineages
                .entry(parent_id)
                .or_insert_with(Vec::new)
                .push(event.id);
        }
    }

    /// 获取子事件 / Get child events
    pub fn get_children(&self, event_id: Uuid) -> Vec<Uuid> {
        self.lineages.get(&event_id).cloned().unwrap_or_default()
    }
}

