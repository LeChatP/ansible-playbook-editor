use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::action::Action;

#[derive(Debug, Serialize, Deserialize, Default, Builder)]
pub struct Become {
    #[serde(rename = "become", deserialize_with = "bool_or_yes", skip_serializing_if = "Option::is_none")]
    pub flag: Option<bool>,
    #[serde(rename = "become_user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "become_method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "become_flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(rename = "become_exe", skip_serializing_if = "Option::is_none")]
    pub exe: Option<String>,
}

fn bool_or_yes<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de> {
    let value: Value = Deserialize::deserialize(deserializer)?;
    if let Some(bool) = value.as_bool() {
        return Ok(Some(bool));
    }
    if let Some(string) = value.as_str() {
        if string == "yes" {
            return Ok(Some(true));
        } else if string == "no" {
            return Ok(Some(false));
        } else {
            return Err(serde::de::Error::custom("expected bool or yes/no"));
        }
    }
    Ok(None)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playbook(pub Vec<Task>);

pub struct PlaybookIterator<'a> {
    playbook: &'a Playbook,
    index: usize,
}

impl<'a> PlaybookIterator<'a> {
    pub fn new(playbook: &'a Playbook) -> Self {
        PlaybookIterator {
            playbook,
            index: 0,
        }
    }
}

impl<'a> Iterator for PlaybookIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.playbook.0.len() {
            let task = &self.playbook.0[self.index];
            self.index += 1;
            Some(task)
        } else {
            None
        }
    }
}

impl Playbook {
    pub fn iter(&self) -> PlaybookIterator {
        PlaybookIterator::new(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Task {
    pub name: String,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub becomes : Option<Become>,
    #[serde(flatten)]
    pub action: Option<Action>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub block: Vec<Box<Task>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<Box<Task>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handlers: Vec<Box<Task>>,
    #[serde(flatten)]
    pub _extra_fields: Value,
}

impl AsRef<Task> for Task {
    fn as_ref(&self) -> &Task {
        self
    }
}