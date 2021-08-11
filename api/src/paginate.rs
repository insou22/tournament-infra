use crate::errors::*;
use data_encoding::BASE64;
use serde::Serialize;
use std::str::FromStr;
use std::fmt::Display;

pub enum Cursor<T> {
    Next(T),
    Prev(T),
    None,
}

pub struct Paginate<T: FromStr> {
    pub per_page: i64,
    pub per_page_with_cursor: i64,
    pub cursor: Cursor<T>,
}

impl<T: FromStr + Display> Paginate<T> {
    pub fn new(per_page: Option<i64>, cursor: Option<String>) -> Result<Self> {
        let per_page = per_page.unwrap_or(50);
        Ok(Self {
            per_page,
            per_page_with_cursor: per_page + 1,
            cursor: match cursor {
                Some(s) => {
                    let s = BASE64.decode(s.as_bytes())?;
                    let s = String::from_utf8(s)?;
                    if let Some(c) = s.get(5..) {
                        let c = c.parse::<T>().or::<Error>(Err("Cursor parse failed.".into()))?;
                        match s.get(..5) {
                            Some(s) => {
                                match s {
                                    "next_" => Cursor::Next(c),
                                    "prev_" => Cursor::Prev(c),
                                    _ => bail!("Invalid cursor type.")
                                }
                            },
                            None => bail!("Invalid cursor type."),
                        }
                    } else {
                        Cursor::None
                    }
                }
                None => Cursor::None,
            },
        })
    }
}

pub trait Paginatable: Serialize {
    type CursorType: FromStr + Display;
    fn get_cursor(&self) -> Self::CursorType;
}

#[derive(Serialize)]
pub struct Paginated<Item: Serialize> {
    pub next_cursor: Option<String>,
    pub prev_cursor: Option<String>,
    pub items: Vec<Item>,
}

impl<Item: Serialize + Paginatable> Paginated<Item> {
    pub fn new(
        mut items: Vec<Item>,
        paginate: Paginate<Item::CursorType>
    ) -> Self {
        let mut next_cursor: Option<Item::CursorType> = None;
        let mut prev_cursor: Option<Item::CursorType> = None;

        match paginate.cursor {
            Cursor::Prev(_) => {
                next_cursor = items.get(0).and_then(|x| Some(x.get_cursor()));

                if let Some(_) = items.get(paginate.per_page_with_cursor as usize - 1) {
                    prev_cursor = items.get(paginate.per_page as usize - 1).and_then(|x| Some(x.get_cursor()));
                }

                items.reverse();
            },
            _ => {
                if let Some(_) = items.get(paginate.per_page_with_cursor as usize - 1) {
                    next_cursor = items.get(paginate.per_page as usize - 1).and_then(|x| Some(x.get_cursor()));
                }

                if let Cursor::Next(_) = paginate.cursor {
                    prev_cursor = items.get(0).and_then(|x| Some(x.get_cursor()));
                }
            }
        }

        items.truncate(paginate.per_page as usize);

        Self {
            next_cursor: next_cursor.and_then(|c| Some(BASE64.encode(format!("next_{}", c).as_bytes()))),
            prev_cursor: prev_cursor.and_then(|c| Some(BASE64.encode(format!("prev_{}", c).as_bytes()))),
            items,
        }
    }
}
