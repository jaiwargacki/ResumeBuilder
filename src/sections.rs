#[derive(serde::Deserialize, serde::Serialize)]
pub struct PersonalInfo {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub linkedin: String,
    pub github: String
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Entry {
    pub institution: String,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub location: String,
    pub description: Vec<String>
}