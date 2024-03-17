// src/parse.rs
use crate::config::{Answer, ExpectList, ExpectOk, ExpectedAnswer, FileProp};
use std::fmt::{Display, Error, Formatter};

impl ExpectedAnswer for ExpectOk {
    fn parse_answer(&self, answer: String) -> Result<Answer, &'static str> {
        if answer == "ok\r\n" {
            ("Réponse correcte du tracker");
            Ok(Answer::Ok)
        } else {
            ("Erreur réponse du tracker: {}", answer);
            Err("Erreur réponse du tracker")
        }
    }
}

impl ExpectedAnswer for ExpectList {
    fn parse_answer(&self, answer: String) -> Result<Answer, &'static str> {
        if !answer.starts_with("list ") {
            ("Erreur réponse du tracker: {}", answer);
            return Err("Erreur réponse du tracker");
        }

        let parts: Vec<&str> = answer.split_whitespace().collect();
        if (parts.len() - 1) % 4 != 0 {
            // The input should start with "list" and then have groups of 4 items
            return Err("Invalid list format");
        }

        let mut props = Vec::new();
        for i in (1..parts.len()).step_by(4) {
            let prop = FileProp {
                file_name: parts[i].to_string(),
                length: parts[i + 1].parse().map_err(|_| "Invalid length")?,
                piece_size: parts[i + 2].parse().map_err(|_| "Invalid piece size")?,
                hash: parts[i + 3].to_string(),
            };
            props.push(prop);
        }
        Ok(Answer::List(props))
    }
}
impl Display for Answer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Answer::Ok => write!(f, "ok"),
            Answer::List(props) => {
                write!(f, "list")?;
                for prop in props {
                    write!(
                        f,
                        " {} {} {} {}",
                        prop.file_name, prop.length, prop.piece_size, prop.hash
                    )?;
                }
                Ok(())
            }
        }
    }
}
/// Verify if the answer match the protocol and return the answer in a well suited format or an error if
/// the answer is not well formatted
pub fn goes_well(
    answer: String,
    expected_answer: &dyn ExpectedAnswer,
) -> Result<Answer, &'static str> {
    expected_answer.parse_answer(answer)
}
