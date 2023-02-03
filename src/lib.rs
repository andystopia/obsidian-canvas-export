use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
struct Edge {
    from_node: String,
    from_side: String,
    id: String,
    to_node: String,
    to_side: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag="type")]
enum Contents {
    #[serde(rename="text")]
    Text {
        text: String
    },
    #[serde(rename="link")]
    Link {
        url: String
    },
    #[serde(rename="file")]
    File {
        file: std::path::PathBuf
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Node {
    x: i64,
    y: i64,
    width: i64,
    height: i64,
    id: String,
    #[serde(flatten)]
    contents: Contents,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Root {
    edges: Vec<Edge>,
    nodes: Vec<Node>,
}

#[derive(Debug)]
pub enum FileDecodingError {
    FileError(std::io::Error),
    DecodingError(serde_json::Error)
}

impl From<serde_json::Error> for FileDecodingError {
    fn from(value: serde_json::Error) -> Self {
        Self::DecodingError(value)
    }
}

impl From<std::io::Error> for FileDecodingError {
    fn from(value: std::io::Error) -> Self {
        Self::FileError(value)
    }
}

impl std::error::Error for FileDecodingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FileDecodingError::FileError(err) => {
                Some(err)
            },
            FileDecodingError::DecodingError(err) => {
                Some(err)
            },
        }
    }
}

impl std::fmt::Display for FileDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileDecodingError::FileError(err) => {
                f.write_fmt(format_args!("File Read Error: {err}"))
            },
            FileDecodingError::DecodingError(err) => {
                f.write_fmt(format_args!("Canvas Decoding Error: {err}"))
            },
        }
    }
}

fn decode_from_json_file<P: AsRef<std::path::Path>>(path: P)  -> Result<Root, FileDecodingError> {
    fn _inner(src: &std::path::Path) -> Result<Root, FileDecodingError> {
        let file = std::fs::OpenOptions::new()
                .read(true)
                .open(src)?;
        let mut bufreader = std::io::BufReader::new(file);

        Ok(serde_json::from_reader(&mut bufreader)?)
    }
    _inner(path.as_ref())

}
use clap::Parser;

#[derive(Debug, Parser)]
enum Program {
    /// Debug Parser
    Debug {
        /// file source
        src: std::path::PathBuf
    },
    /// Compile a File into HTML
    Compile {
        /// file source
        src: std::path::PathBuf
    }
}



pub fn lib_main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    let program = Program::parse();

    match program {
        Program::Debug { src } => {
            let root = decode_from_json_file(src);
            eprintln!("root = {:#?}", root);
        }
        Program::Compile { src } => {
            let root = decode_from_json_file(src)?;

            let cmark_spec = pulldown_cmark::Options::all();


            for node in root.nodes {
                match node.contents {
                    Contents::Text { text } => {


                        let parser = pulldown_cmark::Parser::new_ext(&text, cmark_spec);

                        let mut output = String::new();
                        pulldown_cmark::html::push_html(&mut output, parser);

                        println!("{}", node.id);
                        println!("{output}")
                    }
                    Contents::Link { url: _ } =>  {

                    },
                    Contents::File { file: _ } =>  {

                    },
                }
            }
        }
    }
    Ok(())
}
