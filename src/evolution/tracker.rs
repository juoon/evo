// 进化记录器 / Evolution tracker
// 记录所有语法和语义的进化历史
// Records the evolutionary history of all syntax and semantics

use crate::grammar::rule::GrammarRule;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
        let mut parents = Vec::new();

        // 查找直接父事件：基于规则相似度和时间顺序 / Find direct parent events: based on rule similarity and temporal order
        for prev_event in &self.event_log {
            // 检查是否有规则被新规则继承 / Check if any rules are inherited by new rules
            for new_rule in &event.delta.added_rules {
                for old_rule in &prev_event.delta.added_rules {
                    // 简单的相似度检查 / Simple similarity check
                    if new_rule.name.contains(&old_rule.name)
                        || old_rule.name.contains(&new_rule.name)
                        || new_rule.name == old_rule.name
                    {
                        if !parents.contains(&prev_event.id) {
                            parents.push(prev_event.id);
                        }
                    }
                }
            }

            // 检查是否有规则被修改（修改的规则对 -> from就是父事件）/ Check if rules are modified
            for (_old_rule, new_rule) in &event.delta.modified_rules {
                for prev_new_rule in &prev_event.delta.added_rules {
                    if prev_new_rule.name == new_rule.name {
                        if !parents.contains(&prev_event.id) {
                            parents.push(prev_event.id);
                        }
                    }
                }
            }
        }

        // 如果没有找到直接父事件，选择最近的同类型事件作为父事件 / If no direct parent found, choose recent same-type event
        if parents.is_empty() && !self.event_log.is_empty() {
            // 查找最近的同类型事件 / Find most recent same-type event
            for prev_event in self.event_log.iter().rev() {
                if prev_event.event_type == event.event_type {
                    parents.push(prev_event.id);
                    break;
                }
            }
        }

        // 如果还是没有，选择最近的事件作为父事件 / If still none, choose most recent event
        if parents.is_empty() && !self.event_log.is_empty() {
            if let Some(last_event) = self.event_log.last() {
                parents.push(last_event.id);
            }
        }

        parents
    }

    /// 回滚到指定事件之前的状态 / Rollback to state before specified event
    pub fn rollback_to(&mut self, event_id: Uuid) -> Result<StateSnapshot, String> {
        // 找到要回滚的事件 / Find event to rollback
        let event_index = self
            .event_log
            .iter()
            .position(|e| e.id == event_id)
            .ok_or_else(|| format!("Event {} not found", event_id))?;

        // 获取回滚后的状态（事件之前的状态）/ Get state after rollback (state before event)
        let rollback_state = self.event_log[event_index].before_state.clone();

        // 移除该事件及其后续事件 / Remove this event and subsequent events
        self.event_log.truncate(event_index);

        // 更新谱系（移除相关关系）/ Update genealogy (remove related relationships)
        self.genealogy.remove_event_and_descendants(event_id);

        Ok(rollback_state)
    }

    /// 获取事件的所有后代事件 / Get all descendant events of an event
    pub fn get_descendants(&self, event_id: Uuid) -> Vec<Uuid> {
        let mut descendants = Vec::new();
        let mut to_process = vec![event_id];

        while let Some(current_id) = to_process.pop() {
            let children = self.genealogy.get_children(current_id);
            for child_id in children {
                if !descendants.contains(&child_id) {
                    descendants.push(child_id);
                    to_process.push(child_id);
                }
            }
        }

        descendants
    }

    /// 获取事件的祖先链 / Get ancestor chain of an event
    pub fn get_ancestors(&self, event_id: Uuid) -> Vec<Uuid> {
        let mut ancestors = Vec::new();

        // 在事件日志中查找父事件 / Find parent events in event log
        if let Some(event) = self.event_log.iter().find(|e| e.id == event_id) {
            let parents = self.find_parent_events(event);
            for parent_id in parents {
                if !ancestors.contains(&parent_id) {
                    ancestors.push(parent_id);
                    // 递归查找祖先 / Recursively find ancestors
                    ancestors.extend(self.get_ancestors(parent_id));
                }
            }
        }

        ancestors
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

    /// 移除事件及其后代关系 / Remove event and its descendant relationships
    pub fn remove_event_and_descendants(&mut self, event_id: Uuid) {
        // 移除该事件作为父的所有关系 / Remove all relationships where this event is parent
        self.lineages.remove(&event_id);

        // 移除该事件作为子节点的关系 / Remove relationships where this event is a child
        for (_parent, children) in self.lineages.iter_mut() {
            children.retain(|&child_id| child_id != event_id);
        }
    }

    /// 获取谱系树结构 / Get genealogy tree structure
    pub fn get_tree_structure(&self, root_id: Uuid) -> serde_json::Value {
        let mut tree = serde_json::json!({
            "event_id": root_id.to_string(),
            "children": []
        });

        let children = self.get_children(root_id);
        let mut children_array = Vec::new();
        for child_id in children {
            children_array.push(self.get_tree_structure(child_id));
        }
        tree["children"] = serde_json::json!(children_array);

        tree
    }
}
