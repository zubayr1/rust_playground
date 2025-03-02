#[derive(Debug)]
pub enum Media {
    Book{
        title: String,
        author: String,
    },
    Movie{
        title: String,
        director: String,
    },
    AudioBook{
        title: String,
    },
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
        match self {
            Media::Book{title, author} => format!("Book: {} by {}", title, author),
            Media::Movie{title, director} => format!("Movie: {} by {}", title, director),
            Media::AudioBook{title} => format!("AudioBook: {}", title),
            Media::Placeholder => "Placeholder".to_string(),
        }
    }
}