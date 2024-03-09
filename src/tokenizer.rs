use serde::{Deserialize, Serialize};

pub fn is_supported() {}

pub fn split_lines(mut lines: String, mut filter: String) -> Vec<Line> {
    let filters: Vec<Box<dyn Fn(&str) -> bool>> = filter.split(",").map(|f: &str| Box::new(|x: &str| x == f)).collect();
    let mut lines = lines.split("\n").filter(|x| {
        filters.iter().any(|filter_function| filter_function(&x))
    }).map(|s| Line {
        name: String::from(&s[0..3]),
        // content: String::from(&s[3..]),
        sub: split_line(&String::from(&s[0..])),
    }).collect();
}

pub fn split_line(mut line: &String) -> Vec<Field> {
    let mut fields: Vec<Field> = vec![];
    if let Some(name) = line.get(..3) {
        for (field_idx, field) in line.split("|").enumerate() {
            if field.contains("^") && (name == "MSH" && field_idx != 1usize || name != "MSH") {
                for (sub_field_idx, sub_field) in field.split("^").enumerate() {
                    let field = Field {
                        name: String::from(name) + "." + &*field_idx.to_string() + "." + &*sub_field_idx.to_string(),
                        content: String::from(sub_field),
                        sub: vec![],
                    };
                    fields.push(field);
                }
            } else {
                let field = Field {
                    name: String::from(name) + "." + &*field_idx.to_string(),
                    content: String::from(field),
                    sub: vec![],
                };
                fields.push(field);
            }
        }
    } else {

    }
    return fields;
}

pub fn split_field() -> Vec<SubField> {
    vec![]
}




pub fn tokenize_line() {}

#[derive(Serialize, Deserialize)]
pub struct Line {
    pub name: String,
    // pub content: String,
    pub sub: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub content: String,
    pub sub: Vec<SubField>,
}

#[derive(Serialize, Deserialize)]
pub struct SubField {
    pub name: String,
    pub content: Vec<String>,
}