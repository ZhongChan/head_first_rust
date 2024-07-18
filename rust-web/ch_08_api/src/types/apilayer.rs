use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIResponse {
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadWordsResponse {
    pub content: String,
    #[serde(rename = "bad_words_total")]
    pub bad_words_total: i64,
    #[serde(rename = "bad_words_list")]
    pub bad_words_list: Vec<BadWordsList>,
    #[serde(rename = "censored_content")]
    pub censored_content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadWordsList {
    pub original: String,
    pub word: String,
    pub deviations: i64,
    pub info: i64,
    pub start: i64,
    pub end: i64,
    pub replaced_len: i64,
}
