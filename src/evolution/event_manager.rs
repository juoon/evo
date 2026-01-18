//! 进化事件管理器 / Evolution Event Manager
//!
//! 负责进化事件的保存、加载、合并、验证等功能
//! Responsible for saving, loading, merging, and validating evolution events

use crate::evolution::tracker::{EvolutionDelta, EvolutionEvent, StateSnapshot};
use crate::grammar::rule::GrammarRule;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// 进化事件管理器 / Evolution event manager
pub struct EvolutionEventManager {
    /// 事件存储目录 / Event storage directory
    events_dir: PathBuf,
}

impl EvolutionEventManager {
    /// 创建新的事件管理器 / Create new event manager
    pub fn new(events_dir: impl AsRef<Path>) -> Self {
        Self {
            events_dir: events_dir.as_ref().to_path_buf(),
        }
    }

    /// 保存进化事件到文件 / Save evolution event to file
    pub fn save_event(&self, event: &EvolutionEvent) -> Result<PathBuf, EventManagerError> {
        // 确保目录存在 / Ensure directory exists
        fs::create_dir_all(&self.events_dir).map_err(|e| EventManagerError::IoError(e))?;

        // 生成文件名 / Generate filename
        let filename = format!("event_{}.json", event.id);
        let filepath = self.events_dir.join(&filename);

        // 序列化为JSON / Serialize to JSON
        let json = serde_json::to_string_pretty(event)
            .map_err(|e| EventManagerError::SerializationError(e))?;

        // 写入文件 / Write to file
        fs::write(&filepath, json).map_err(|e| EventManagerError::IoError(e))?;

        Ok(filepath)
    }

    /// 从文件加载进化事件 / Load evolution event from file
    pub fn load_event(&self, event_id: Uuid) -> Result<EvolutionEvent, EventManagerError> {
        let filename = format!("event_{}.json", event_id);
        let filepath = self.events_dir.join(&filename);

        // 读取文件 / Read file
        let content = fs::read_to_string(&filepath).map_err(|e| EventManagerError::IoError(e))?;

        // 反序列化 / Deserialize
        let event: EvolutionEvent = serde_json::from_str(&content)
            .map_err(|e| EventManagerError::DeserializationError(e))?;

        Ok(event)
    }

    /// 加载所有进化事件 / Load all evolution events
    pub fn load_all_events(&self) -> Result<Vec<EvolutionEvent>, EventManagerError> {
        let mut events = Vec::new();

        // 确保目录存在 / Ensure directory exists
        if !self.events_dir.exists() {
            return Ok(events);
        }

        // 读取目录中的所有JSON文件 / Read all JSON files in directory
        let entries = fs::read_dir(&self.events_dir).map_err(|e| EventManagerError::IoError(e))?;

        for entry in entries {
            let entry = entry.map_err(|e| EventManagerError::IoError(e))?;
            let path = entry.path();

            // 只处理JSON文件 / Only process JSON files
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(event) = serde_json::from_str::<EvolutionEvent>(&content) {
                        events.push(event);
                    }
                }
            }
        }

        // 按时间戳排序 / Sort by timestamp
        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(events)
    }

    /// 验证进化事件 / Validate evolution event
    pub fn validate_event(&self, event: &EvolutionEvent) -> Result<(), EventValidationError> {
        // 1. 检查基本字段 / Check basic fields
        if event.id == Uuid::nil() {
            return Err(EventValidationError::InvalidId);
        }

        // 2. 检查状态快照一致性 / Check state snapshot consistency
        if event.before_state.version.is_empty() {
            return Err(EventValidationError::InvalidState);
        }

        // 3. 检查delta的有效性 / Check delta validity
        if event.delta.added_rules.is_empty()
            && event.delta.modified_rules.is_empty()
            && event.delta.removed_rules.is_empty()
        {
            return Err(EventValidationError::EmptyDelta);
        }

        // 4. 检查规则有效性 / Check rule validity
        for rule in &event.delta.added_rules {
            if rule.name.is_empty() {
                return Err(EventValidationError::InvalidRule);
            }
        }

        Ok(())
    }

    /// 检测进化事件冲突 / Detect conflicts between evolution events
    pub fn detect_conflicts(&self, events: &[EvolutionEvent]) -> Vec<EventConflict> {
        let mut conflicts = Vec::new();

        for i in 0..events.len() {
            for j in (i + 1)..events.len() {
                if let Some(conflict) = self.check_event_conflict(&events[i], &events[j]) {
                    conflicts.push(conflict);
                }
            }
        }

        conflicts
    }

    /// 检查两个事件是否冲突 / Check if two events conflict
    fn check_event_conflict(
        &self,
        event1: &EvolutionEvent,
        event2: &EvolutionEvent,
    ) -> Option<EventConflict> {
        // 检查是否修改相同的规则 / Check if modifying same rules
        let mut conflicting_rules = Vec::new();

        // 检查添加的规则 / Check added rules
        for rule1 in &event1.delta.added_rules {
            for rule2 in &event2.delta.added_rules {
                if rule1.name == rule2.name {
                    conflicting_rules.push(rule1.name.clone());
                }
            }
        }

        // 检查修改的规则 / Check modified rules
        for (old1, new1) in &event1.delta.modified_rules {
            for (old2, new2) in &event2.delta.modified_rules {
                if old1.name == old2.name || new1.name == new2.name {
                    conflicting_rules.push(new1.name.clone());
                }
            }
        }

        // 检查删除的规则 / Check removed rules
        for rule1 in &event1.delta.removed_rules {
            for rule2 in &event2.delta.removed_rules {
                if rule1.name == rule2.name {
                    conflicting_rules.push(rule1.name.clone());
                }
            }
        }

        if !conflicting_rules.is_empty() {
            Some(EventConflict {
                event1_id: event1.id,
                event2_id: event2.id,
                conflicting_rules,
                conflict_type: ConflictType::RuleModification,
            })
        } else {
            None
        }
    }

    /// 合并进化事件 / Merge evolution events
    pub fn merge_events(
        &self,
        events: Vec<EvolutionEvent>,
    ) -> Result<EvolutionEvent, EventManagerError> {
        if events.is_empty() {
            return Err(EventManagerError::EmptyEventList);
        }

        // 1. 检测冲突 / Detect conflicts
        let conflicts = self.detect_conflicts(&events);
        if !conflicts.is_empty() {
            // 如果有冲突，选择最优事件 / If conflicts exist, select best event
            return self.select_best_event(events, &conflicts);
        }

        // 2. 按时间戳排序 / Sort by timestamp
        let mut sorted_events = events;
        sorted_events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // 3. 合并所有delta / Merge all deltas
        let mut merged_delta = EvolutionDelta {
            added_rules: Vec::new(),
            modified_rules: Vec::new(),
            removed_rules: Vec::new(),
            description: String::new(),
        };

        let mut descriptions = Vec::new();
        let mut base_version = String::new();

        for event in &sorted_events {
            if base_version.is_empty() {
                base_version = event.before_state.version.clone();
            }

            merged_delta
                .added_rules
                .extend(event.delta.added_rules.clone());
            merged_delta
                .modified_rules
                .extend(event.delta.modified_rules.clone());
            merged_delta
                .removed_rules
                .extend(event.delta.removed_rules.clone());
            descriptions.push(event.delta.description.clone());
        }

        merged_delta.description = format!(
            "Merged {} events: {}",
            sorted_events.len(),
            descriptions.join("; ")
        );

        // 4. 构建合并后的事件 / Build merged event
        let first_event = &sorted_events[0];
        let last_event = sorted_events.last().unwrap();

        let merged_event = EvolutionEvent {
            id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_type: first_event.event_type.clone(),
            before_state: first_event.before_state.clone(),
            after_state: last_event.after_state.clone(),
            delta: merged_delta,
            trigger: first_event.trigger.clone(),
            author: None,
            success_metrics: self.merge_metrics(&sorted_events),
        };

        Ok(merged_event)
    }

    /// 选择最优事件（当有冲突时）/ Select best event (when conflicts exist)
    fn select_best_event(
        &self,
        events: Vec<EvolutionEvent>,
        _conflicts: &[EventConflict],
    ) -> Result<EvolutionEvent, EventManagerError> {
        // 基于成功指标选择最优事件 / Select best event based on success metrics
        let mut best_event = None;
        let mut best_score = f64::NEG_INFINITY;

        for event in events {
            let score = self.calculate_event_score(&event);
            if score > best_score {
                best_score = score;
                best_event = Some(event);
            }
        }

        best_event.ok_or(EventManagerError::EmptyEventList)
    }

    /// 计算事件评分 / Calculate event score
    fn calculate_event_score(&self, event: &EvolutionEvent) -> f64 {
        if let Some(metrics) = &event.success_metrics {
            metrics.success_rate * 0.4
                + metrics.performance_improvement * 0.3
                + (1.0 - metrics.compatibility_impact.abs()) * 0.3
        } else {
            0.5 // 默认分数 / Default score
        }
    }

    /// 合并指标 / Merge metrics
    fn merge_metrics(
        &self,
        events: &[EvolutionEvent],
    ) -> Option<crate::evolution::tracker::EvolutionMetrics> {
        if events.is_empty() {
            return None;
        }

        let mut total_success_rate = 0.0;
        let mut total_performance = 0.0;
        let mut total_compatibility = 0.0;
        let mut count = 0;

        for event in events {
            if let Some(metrics) = &event.success_metrics {
                total_success_rate += metrics.success_rate;
                total_performance += metrics.performance_improvement;
                total_compatibility += metrics.compatibility_impact;
                count += 1;
            }
        }

        if count > 0 {
            Some(crate::evolution::tracker::EvolutionMetrics {
                success_rate: total_success_rate / count as f64,
                user_satisfaction_delta: 0.0,
                performance_improvement: total_performance / count as f64,
                compatibility_impact: total_compatibility / count as f64,
            })
        } else {
            None
        }
    }
}

/// 事件管理器错误 / Event manager error
#[derive(Debug)]
pub enum EventManagerError {
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    DeserializationError(serde_json::Error),
    EmptyEventList,
    ValidationError(EventValidationError),
}

impl std::fmt::Display for EventManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventManagerError::IoError(e) => write!(f, "IO error: {}", e),
            EventManagerError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            EventManagerError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
            EventManagerError::EmptyEventList => write!(f, "Empty event list"),
            EventManagerError::ValidationError(e) => write!(f, "Validation error: {:?}", e),
        }
    }
}

impl std::error::Error for EventManagerError {}

/// 事件验证错误 / Event validation error
#[derive(Debug, Clone)]
pub enum EventValidationError {
    InvalidId,
    InvalidState,
    EmptyDelta,
    InvalidRule,
}

/// 事件冲突 / Event conflict
#[derive(Debug, Clone)]
pub struct EventConflict {
    pub event1_id: Uuid,
    pub event2_id: Uuid,
    pub conflicting_rules: Vec<String>,
    pub conflict_type: ConflictType,
}

/// 冲突类型 / Conflict type
#[derive(Debug, Clone)]
pub enum ConflictType {
    RuleModification,
    StateIncompatibility,
    PerformanceRegression,
}
