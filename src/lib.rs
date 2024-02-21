use std::fmt::{Display, Formatter, Result};

pub trait Greet {
    type Output;

    fn hello_world(&self) -> Self::Output;
}

pub struct Programmer {
    pub names: Names,
    pub history: Vec<HistoryItem>,
}

impl Greet for Programmer {
    type Output = String;


    fn hello_world(&self) -> String {
        let mut history = self.history.iter().map(|item| match item {
            HistoryItem::Programming(lang, _years) => lang.to_owned(),
            HistoryItem::Etc => "...".to_owned(),
        });

        let mut output = if let Some(item) = history.next() {
            format!(
                "Hello! world, My name is {}\nI have previously programmed in: {}",
                self.names, item
            )
        } else {
            return format!(
                "Hello! world, My name is {}\nI have no prior programming experience.",
                self.names
            );
        };

        for item in history {
            output.push_str(", ");
            output.push_str(&item);
        }

        output
    }
}

#[non_exhaustive]
pub enum HistoryItem {
    Programming(String, u8),
    Etc,
}

pub struct Names {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

impl Display for Names {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.middle_name {
            Some(middle_name) => write!(
                f,
                "{} {} {}",
                self.first_name, middle_name, self.last_name
            ),
            None => write!(f, "{} {}", self.first_name, self.last_name),
        }
    }
}