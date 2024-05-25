#[derive(serde::Deserialize, serde::Serialize)]
pub struct Resume {
    pub personal_info: PersonalInfo,
    pub bio: String,
    pub skills: Vec<String>,
    pub experience: Vec<Entry>,
    pub education: Vec<Entry>,
}

impl Default for Resume {
    fn default() -> Self {
        Self {
            personal_info: PersonalInfo {
                name: "".to_owned(),
                email: "".to_owned(),
                phone: "".to_owned(),
                location: "".to_owned(),
                linkedin: "".to_owned(),
                github: "".to_owned()
            },
            bio: "".to_owned(),
            skills: vec![],
            experience: vec![],
            education: vec![],
        }
    }
}

impl std::fmt::Debug for Resume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Resume")
            .field("personal_info", &self.personal_info)
            .field("bio", &self.bio)
            .field("skills", &self.skills)
            .field("experience", &self.experience)
            .field("education", &self.education)
            .finish()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PersonalInfo {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub linkedin: String,
    pub github: String
}

impl Default for PersonalInfo {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            email: "".to_owned(),
            phone: "".to_owned(),
            location: "".to_owned(),
            linkedin: "".to_owned(),
            github: "".to_owned()
        }
    }
}

impl std::fmt::Debug for PersonalInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PersonalInfo")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("phone", &self.phone)
            .field("location", &self.location)
            .field("linkedin", &self.linkedin)
            .field("github", &self.github)
            .finish()
    }
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

impl Default for Entry {
    fn default() -> Self {
        Self {
            institution: "".to_owned(),
            title: "".to_owned(),
            start_date: "".to_owned(),
            end_date: "".to_owned(),
            location: "".to_owned(),
            description: vec![]
        }
    }
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("institution", &self.institution)
            .field("title", &self.title)
            .field("start_date", &self.start_date)
            .field("end_date", &self.end_date)
            .field("location", &self.location)
            .field("description", &self.description)
            .finish()
    }
}