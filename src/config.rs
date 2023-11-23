use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use std::{collections::BTreeMap, path::PathBuf};
use url::Url;

#[derive(Deserialize, Debug)]
pub struct InnerConfig {
    pub shell: Option<String>,
    pub schemes: Option<Schemes>,
    pub templates: Option<Templates>,
    pub item: Option<BTreeMap<String, ConfigItem>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Schemes {
    Vec(Vec<SchemeSource>),
    Map(LinkedHashMap<String, SchemeSource>),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SchemeSource {
    Simple(Url),
    Detailed(SchemeSourceDetailed),
}

#[derive(Deserialize, Debug)]
pub struct SchemeSourceDetailed {
    pub source: Url,
    pub files: Option<Vec<PathBuf>>,
    pub repo_type: Option<SchemeRepoType>,
    pub reference: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SchemeRepoType {
    Simple(SchemeRepoTypeSimple),
    Detailed(SchemeRepoTypeDetailed),
}

#[derive(Deserialize, Debug)]
#[serde(rename = "snake_case")]
pub enum SchemeRepoTypeSimple {
    SourceList,
    MonoRepo,
}

#[derive(Deserialize, Debug)]
pub struct SchemeRepoTypeDetailed {
    #[serde(default)]
    pub source_list: bool,
    #[serde(default)]
    pub mono_repo: bool,
}

#[derive(Deserialize, Debug)]
pub struct ConfigItem {
    pub schema: SchemaNameOrSource,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SchemaNameOrSource {
    SourceSimple(Url),
    Name(String),
    SourceDetailed(SchemeSourceDetailed),
}

#[derive(Deserialize, Debug)]
pub enum Templates {}
