use std::{fmt, marker::PhantomData};

use bon::Builder;
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Deserializer, Serialize};
use serde_yaml::Value;

use crate::file::Become;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Role {
    pub name: String,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub becomes : Option<Become>,
    #[builder(default)]
    #[serde(flatten)]
    pub _extra_fields: Value,
}


#[derive(Debug)]
pub struct ImportRole {
    name: String,
    pub role: Role,
}

struct ImportRoleVisitor {
    marker: PhantomData<fn() -> ImportRole>
}

impl ImportRoleVisitor {
    fn new() -> Self {
        ImportRoleVisitor {
            marker: PhantomData
        }
    }
}

impl<'de> Visitor<'de> for ImportRoleVisitor
{
    // The type that our Visitor is going to produce.
    type Value = ImportRole;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a role structure containing a name")
    }
    
    
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name: Option<String> = None;
        let mut map = map;
        while let Some(key) = map.next_key()? {
            match key {
                "name" => {
                    if name.is_some() {
                        return Err(serde::de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(key, &["name"]));
                }
            }
        }
        let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
        let role = Role::builder().name(name.clone()).build();
        Ok(ImportRole {
            name,
            role
        })
    }
    
}

impl<'de> Deserialize<'de> for ImportRole
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ImportRoleVisitor::new())
    }
}

impl Serialize for ImportRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("name", &self.name)?;
        map.end()
    }
}