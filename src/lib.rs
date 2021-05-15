use serde::Deserialize;
use serde_json::Value;

type Map = serde_json::Map<String, Value>;

#[derive(Deserialize, Debug)]
pub struct Notebook {
    metadata: Map,

    #[serde(flatten)]
    format: Format,

    cells: Vec<Cell>,
}

#[derive(Deserialize, Debug)]
pub struct Format {
    #[serde(rename = "nbformat")]
    major: u32,

    #[serde(rename = "nbformat_minor")]
    minor: u32,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cell_type", rename_all = "lowercase")]
pub enum Cell {
    Markdown {
        metadata: Map,
        source: Vec<String>,
        attachments: Map,
    },
    Code {
        metadata: Map,
        source: Vec<String>,
        execution_count: Option<u32>,
        outputs: Vec<Output>,
    },
    Raw {
        metadata: Map,
        source: Vec<String>,
        attachments: Map,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "output_type", rename_all = "snake_case")]
pub enum Output {
    Stream {
        name: String,
        text: Vec<String>,
    },
    Display {
        data: String,
        metadata: String,
    },
    ExecuteResult {
        execution_count: u32,
        data: Map,
        metadata: Map,
    },
    Error {
        ename: String,
        evalue: String,
        traceback: Vec<String>,
    },
}

