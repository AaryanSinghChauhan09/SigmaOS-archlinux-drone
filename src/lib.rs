//! SigmaOS-archlinux-drone: Drone CI config model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for Drone CI config.
pub struct DronePipeline {
    pub id: String,
    pub name: String,
    pub trigger: String,
    pub enabled: bool,
}

/// Secondary domain entity for Drone CI config.
pub struct DroneStep {
    pub id: String,
    pub image: String,
    pub description: String,
}

/// Errors for the Drone CI config domain.
#[derive(Debug, PartialEq)]
pub enum DroneError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for DroneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for DroneError {}

/// Registry and manager for Drone CI config.
pub struct DroneManager {
    pub name: String,
    items: BTreeMap<String, DronePipeline>,
    configs: BTreeMap<String, DroneStep>,
}

impl DroneManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: DronePipeline) -> Result<(), DroneError> {
        if item.name.is_empty() {
            return Err(DroneError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(DroneError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&DronePipeline> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: DroneStep) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), DroneError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| DroneError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_DronePipeline(id: &str, name: &str) -> DronePipeline {
        DronePipeline {
            id: id.to_string(),
            name: name.to_string(),
            trigger: "archlinux-drone-default".to_string(),
            enabled: true,
        }
    }

    fn make_DroneStep(id: &str, val: &str) -> DroneStep {
        DroneStep {
            id: id.to_string(),
            image: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = DroneManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = DroneManager::new("test");
        let item = make_DronePipeline("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = DroneManager::new("test");
        let item = make_DronePipeline("item1", "");
        assert_eq!(mgr.add_item(item), Err(DroneError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = DroneManager::new("test");
        let item1 = make_DronePipeline("item1", "First");
        let item2 = make_DronePipeline("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(DroneError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = DroneManager::new("test");
        let cfg = make_DroneStep("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = DroneManager::new("test");
        let item = make_DronePipeline("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = DroneManager::new("test");
        let item = make_DronePipeline("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(DroneError::NotFound("none".to_string())));
    }
}
