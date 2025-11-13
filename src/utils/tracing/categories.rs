use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct Categories(pub Vec<String>);

impl Serialize for Categories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let joined = self.0.join(",");
        serializer.serialize_str(&joined)
    }
}
