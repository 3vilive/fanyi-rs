use serde::{Deserialize, Serialize};
use serde_xml_rs;
use std::fmt;
use std::fmt::Display;
use url::form_urlencoded;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Statement {
    orig: String,
    trans: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RespItem {
    Key(String),
    PS(String),
    Pron(String),
    Pos(String),
    Acceptation(String),

    #[serde(rename = "sent")]
    Statement(Statement),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "dict")]
pub struct RespDict {
    #[serde(rename = "$value")]
    items: Vec<RespItem>,
}

#[derive(Debug)]
struct PhoneticSymbol {
    symbol: String,
    pronunciation_url: String,
}

#[derive(Debug)]
pub struct HandyDict {
    pub key: String,
    pub phonetic_symbols: Vec<String>,
    pub phonetic_symbols_pronunciations: Vec<String>,
    pub part_of_speech: String,
    pub meaning: String,
    pub statements: Vec<Statement>,
}

impl HandyDict {
    pub fn new_from_dict(dict: &RespDict) -> HandyDict {
        let mut handy_dict = HandyDict {
            key: String::new(),
            phonetic_symbols: Vec::with_capacity(2),
            phonetic_symbols_pronunciations: Vec::with_capacity(2),
            meaning: String::new(),
            part_of_speech: String::new(),
            statements: Vec::with_capacity(5),
        };

        for item in dict.items.iter() {
            match item {
                RespItem::Key(key) => handy_dict.key = key.clone(),
                RespItem::Acceptation(meaning) => handy_dict.meaning = meaning.clone(),
                RespItem::Pos(pos) => handy_dict.part_of_speech = pos.clone(),
                RespItem::PS(phonetic_symbol) => {
                    handy_dict.phonetic_symbols.push(phonetic_symbol.clone())
                }
                RespItem::Pron(pronunciation) => handy_dict
                    .phonetic_symbols_pronunciations
                    .push(pronunciation.clone()),
                RespItem::Statement(statement) => handy_dict.statements.push(statement.clone()),
            }
        }

        return handy_dict;
    }
}

impl Display for HandyDict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n {}", self.key)?;

        if self.phonetic_symbols.len() >= 2 {
            write!(
                f,
                " 英[ {} ] 美[ {} ]",
                self.phonetic_symbols[0], self.phonetic_symbols[1]
            )?
        }

        write!(f, " ~ iciba.com\n\n")?;

        if self.part_of_speech != "" {
            write!(f, " - {} {}\n\n", self.part_of_speech, self.meaning)?;
        }
        for (index, stat) in self.statements.iter().enumerate() {
            write!(f, " {}. {}\n   {}\n", index + 1, stat.orig, stat.trans)?
        }

        Ok(())
    }
}

async fn get_translate_resp_body(content: &str) -> Result<String, reqwest::Error> {
    let encoded_q = form_urlencoded::Serializer::new(String::new())
        .append_pair("w", content)
        .finish();

    let url = format!(
        "http://dict-co.iciba.com/api/dictionary.php?key=D191EBD014295E913574E1EAF8E06666&{}",
        encoded_q
    );

    reqwest::get(&url).await?.text().await
}

pub async fn get_translate_result(content: &str) -> Result<HandyDict, String> {
    get_translate_resp_body(content).await
        .map_err(|err| format!("{}", err))
        .and_then(|body| {
            serde_xml_rs::from_str::<RespDict>(&body)
                .map_err(|err| format!("{}", err))
                .map(|resp_dict| HandyDict::new_from_dict(&resp_dict))
        })
}
