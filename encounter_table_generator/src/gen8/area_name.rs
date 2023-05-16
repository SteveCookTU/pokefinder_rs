use serde::Deserialize;

#[derive(Deserialize)]
pub struct WordData {
    pub str: String,
}

#[derive(Deserialize)]
pub struct LabelData {
    #[serde(rename = "labelName")]
    pub label_name: String,
    #[serde(rename = "wordDataArray")]
    pub word_data_array: Vec<WordData>,
}

#[derive(Deserialize)]
pub struct AreaName {
    #[serde(rename = "labelDataArray")]
    pub label_data_array: Vec<LabelData>,
}
