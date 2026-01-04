use crate::utils::content::Post;
use leptos::prelude::*;

#[derive(Clone)]
pub struct AboutResource(pub LocalResource<Option<Post>>);

#[derive(Clone)]
pub struct ProjectsResource(pub LocalResource<Option<Post>>);
