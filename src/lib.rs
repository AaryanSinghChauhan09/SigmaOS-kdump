// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-kdump: kdump component model.
//!
//! Real implementation: kdump_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KdumpItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KdumpError {
    EmptyKdumpItemName,
    DuplicateKdumpItem(String),
    EmptyEventSubject,
}

impl fmt::Display for KdumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKdumpItemName => write!(f, "kdump_item name must not be empty"),
            Self::DuplicateKdumpItem(id) => write!(f, "duplicate kdump_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for KdumpError {}

pub struct Kdump {
    pub name: String,
    kdump_items: BTreeMap<String, KdumpItem>,
    events: BTreeMap<String, Event>,
}

impl Kdump {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kdump_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_kdump_item(&mut self, item: KdumpItem) -> Result<(), KdumpError> {
        if item.name.trim().is_empty() {
            return Err(KdumpError::EmptyKdumpItemName);
        }
        let id = item.id.clone();
        if self.kdump_items.contains_key(&id) {
            return Err(KdumpError::DuplicateKdumpItem(id.clone()));
        }
        self.kdump_items.insert(id, item);
        Ok(())
    }

    pub fn kdump_item_count(&self) -> usize {
        self.kdump_items.len()
    }

    pub fn get_kdump_item(&self, id: &str) -> Option<&KdumpItem> {
        self.kdump_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), KdumpError> {
        if event.subject.trim().is_empty() {
            return Err(KdumpError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kdump_item(id: &str, name: &str) -> KdumpItem {
        KdumpItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Kdump::new("Kdump");
        assert_eq!(m.name, "Kdump");
        assert_eq!(m.kdump_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_kdump_item() {
        let mut m = Kdump::new("Kdump");
        m.add_kdump_item(sample_kdump_item("a1", "Alpha")).unwrap();
        assert_eq!(m.kdump_item_count(), 1);
        assert!(m.get_kdump_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Kdump::new("Kdump");
        assert!(m.add_kdump_item(sample_kdump_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Kdump::new("Kdump");
        m.add_kdump_item(sample_kdump_item("x", "One")).unwrap();
        assert!(m.add_kdump_item(sample_kdump_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Kdump::new("Kdump");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
