use serde::{Deserialize, de::Visitor};

const ASSETS: &str = "assets/img";

#[derive(Debug)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub image: String,
}

impl Unit {
    pub fn image(&self, origin: &str) -> String {
        format!("{origin}/{}", self.image)
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Id,
            Name,
            Image,
        }

        struct UnitVisitor;

        impl<'de> Visitor<'de> for UnitVisitor {
            type Value = Unit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("expected a Unit")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut image: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Image => {
                            if image.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image = Some(map.next_value()?);
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let image = image.ok_or_else(|| serde::de::Error::missing_field("image"))?;

                let image = format!("{ASSETS}/{image}");

                Ok(Unit { id, name, image })
            }
        }
        deserializer.deserialize_struct("Unit", &["id", "name", "image"], UnitVisitor)
    }
}
