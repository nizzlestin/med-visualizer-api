use crate::Filter;
use serde::{Deserialize, Serialize};

pub fn split_lines(mut lines: String, mut filter: Filter) -> Vec<Line> {
    let mut segment_filters: Vec<Box<dyn Fn(String) -> bool>> = vec![];
    if let Some(filter) = &filter.segment_filter {
        for f in filter.split(",") {
            segment_filters.push(Box::new(|x: String| x.starts_with(&f.to_string())));
        }
    } else {
        segment_filters.push(Box::new(|x: String| true));
    }

    let field_filter = filter.field_filter.clone();
    let mut lines = lines
        .split("\n")
        .filter(|x| x.len() > 2)
        .filter(|x| {
            segment_filters
                .iter()
                .any(|filter_function| filter_function(x.to_string()))
        })
        .map(|s| Line {
            name: String::from(&s[0..3]),
            sub: split_line(&String::from(&s[0..]), field_filter.clone()),
        })
        .collect();
    return lines;
}

pub fn split_line(mut line: &String, filter: Option<String>) -> Vec<Field> {
    let mut fields: Vec<Field> = vec![];
    if let Some(name) = line.get(..3) {
        for (field_idx, field) in line.split("|").enumerate() {
            if field.contains("^") && (name == "MSH" && field_idx != 1usize || name != "MSH") {
                for (sub_field_idx, sub_field) in field.split("^").enumerate() {
                    let field = Field {
                        name: String::from(name)
                            + "."
                            + &*field_idx.to_string()
                            + "."
                            + &*sub_field_idx.to_string(),
                        content: String::from(sub_field),
                    };
                    fields.push(field);
                }
            } else {
                let field = Field {
                    name: String::from(name) + "." + &*field_idx.to_string(),
                    content: String::from(field),
                };
                fields.push(field);
            }
        }
    } else {
    }
    let mut final_fields = vec![];
    let mut field_filters: Vec<Box<dyn Fn(String) -> bool>> = vec![];
    if let Some(filter) = &filter {
        for f in filter.split(",") {
            field_filters.push(Box::new(|x: String| x.starts_with(&f.to_string())));
        }
    } else {
        field_filters.push(Box::new(|x: String| true));
    }
    for field in fields {
        if field_filters
            .iter()
            .any(|filter_function| filter_function(field.name.to_string()))
        {
            final_fields.push(field);
        }
    }
    return final_fields;
}

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
}
