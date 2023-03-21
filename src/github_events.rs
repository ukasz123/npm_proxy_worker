use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct PushEvent {
    #[serde(rename= "ref")]
    pub branch_ref: String,
    pub head_commit: Commit,
    pub commits: Vec<Commit>,
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
