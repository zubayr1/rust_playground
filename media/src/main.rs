#[derive(Debug)]
enum Media {
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
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book{title, author} => format!("Book: {} by {}", title, author),
            Media::Movie{title, director} => format!("Movie: {} by {}", title, director),
            Media::AudioBook{title} => format!("AudioBook: {}", title),
        }
    }
}

fn print_media(media: &Media) {
    match media {
        Media::Book{title, author} => println!("Book: {} by {}", title, author),
        Media::Movie{title, director} => println!("Movie: {} by {}", title, director),
        Media::AudioBook{title} => println!("AudioBook: {}", title),
    }
}


fn main() {
    let book = Media::Book{
        title: "The Hobbit".to_string(),
        author: "J.R.R. Tolkien".to_string(),
    };

    print_media(&book);
}
