use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "blogs"]
pub struct Blog {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

// any type that implements Into<String> can be used to create Invitation
impl Blog {
    pub fn from_details<S: Into<String>, T: Into<String>, U: Into<String>>(
        uuid: S,
        title: T,
        content: U,
    ) -> Self {
        Blog {
            id: uuid.into(),
            title: title.into(),
            content: content.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
