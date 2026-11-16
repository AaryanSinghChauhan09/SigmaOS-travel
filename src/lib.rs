// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-travel: travel component model.
//!
//! Real implementation: travel_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TravelItem {
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
pub enum TravelError {
    EmptyTravelItemName,
    DuplicateTravelItem(String),
    EmptyEventSubject,
}

impl fmt::Display for TravelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTravelItemName => write!(f, "travel_item name must not be empty"),
            Self::DuplicateTravelItem(id) => write!(f, "duplicate travel_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for TravelError {}

pub struct Travel {
    pub name: String,
    travel_items: BTreeMap<String, TravelItem>,
    events: BTreeMap<String, Event>,
}

impl Travel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            travel_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_travel_item(&mut self, item: TravelItem) -> Result<(), TravelError> {
        if item.name.trim().is_empty() {
            return Err(TravelError::EmptyTravelItemName);
        }
        let id = item.id.clone();
        if self.travel_items.contains_key(&id) {
            return Err(TravelError::DuplicateTravelItem(id.clone()));
        }
        self.travel_items.insert(id, item);
        Ok(())
    }

    pub fn travel_item_count(&self) -> usize {
        self.travel_items.len()
    }

    pub fn get_travel_item(&self, id: &str) -> Option<&TravelItem> {
        self.travel_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), TravelError> {
        if event.subject.trim().is_empty() {
            return Err(TravelError::EmptyEventSubject);
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

    fn sample_travel_item(id: &str, name: &str) -> TravelItem {
        TravelItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Travel::new("Travel");
        assert_eq!(m.name, "Travel");
        assert_eq!(m.travel_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_travel_item() {
        let mut m = Travel::new("Travel");
        m.add_travel_item(sample_travel_item("a1", "Alpha")).unwrap();
        assert_eq!(m.travel_item_count(), 1);
        assert!(m.get_travel_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Travel::new("Travel");
        assert!(m.add_travel_item(sample_travel_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Travel::new("Travel");
        m.add_travel_item(sample_travel_item("x", "One")).unwrap();
        assert!(m.add_travel_item(sample_travel_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Travel::new("Travel");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
