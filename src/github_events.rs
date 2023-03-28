use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct PushEvent {
    #[serde(rename= "ref")]
    pub branch_ref: String,
    pub head_commit: Commit,
    pub commits: Vec<Commit>,
    pub repository: Repository,
}   


#[derive(Debug, Deserialize)]
pub struct Commit {
    pub id: String,
    
    #[serde(default)]
    pub added: Vec<String>,
    #[serde(default)]
    pub modified: Vec<String>,
    #[serde(default)]
    pub removed: Vec<String>,

    pub url: String,

    pub message: String,

}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: u32,

    pub node_id: String,

    pub name: String,

    pub full_name: String,

    pub private: bool,

    pub html_url: String,

    // API urls
    //TODO: deserialize the pattern into specialized trait
    pub contents_url: String,
}
