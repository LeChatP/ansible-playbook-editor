use std::{fmt, marker::PhantomData, path::PathBuf};

use bon::bon;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use serde_yaml::Value;

use crate::{file::Playbook, role::ImportRole};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    #[serde(alias = "ansible.builtin.include_tasks")]
    IncludeTasks(ImportPlaybook),
    #[serde(alias = "ansible.builtin.import_playbook")]
    ImportPlaybook(ImportPlaybook),
    #[serde(alias = "ansible.builtin.import_role")]
    ImportRole(ImportRole),

    Other(Value)
}

#[bon]
impl Action {
    #[builder]
    pub fn include_tasks(file: PathBuf) -> Self {
        let content = std::fs::read_to_string(&file).unwrap();
        
        let playbook: Playbook = serde_yaml::from_str(&content).unwrap();
        Action::IncludeTasks(ImportPlaybook { file, playbook })
    }

    pub fn write(&self) {
        match self {
            Action::IncludeTasks(task) => {
                let content = serde_yaml::to_string(&task.playbook).unwrap();
                std::fs::write(&task.file, content).unwrap();
            },
            _ => {
                todo!()
            }
        }
    }
}


#[derive(Debug)]
pub struct ImportPlaybook {
    pub file: PathBuf,
    pub playbook: Playbook,
}

struct ImportPlaybookVisitor {
    marker: PhantomData<fn() -> ImportPlaybook>
}

impl ImportPlaybookVisitor {
    fn new() -> Self {
        ImportPlaybookVisitor {
            marker: PhantomData
        }
    }
}

impl<'de> Visitor<'de> for ImportPlaybookVisitor
{
    // The type that our Visitor is going to produce.
    type Value = ImportPlaybook;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let content = std::fs::read_to_string(&v).unwrap();
        println!("reading file: {}", v);
        let playbook: Playbook = serde_yaml::from_str(&content).unwrap();
        Ok(ImportPlaybook {
            file: v.into(),
            playbook
        })
    }
}

impl<'de> Deserialize<'de> for ImportPlaybook
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ImportPlaybookVisitor::new())
    }
}

impl Serialize for ImportPlaybook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.file.as_os_str().to_str().unwrap())
    }
}
