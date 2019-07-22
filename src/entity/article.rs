use crate::entity::{form::article_editor::{Form, Field}, Author, Timestamp, Markdown, Tag};
use crate::entity::article::tag::IntoStrings;
use slug::Slug;

pub mod feed;
pub mod slug;
pub mod tag;
pub mod comment;

#[derive(Clone)]
pub struct Article {
    pub title: String,
    pub slug: Slug,
    pub body: Markdown,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub tag_list: Vec<Tag>,
    pub description: String,
    pub author: Author<'static>,
    pub favorited: bool,
    pub favorites_count: usize,
}

impl Article {
    pub fn into_form(self) -> Form {
        let fields: Vec<Field> = vec![
            Field::Title(self.title),
            Field::Description(self.description),
            Field::Body(self.body.to_string()),
            Field::Tags(self.tag_list.into_strings().join(" ")),
        ];
        Form::new(fields)
    }
}