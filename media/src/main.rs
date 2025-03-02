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
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book{title, author} => format!("Book: {} by {}", title, author),
            Media::Movie{title, director} => format!("Movie: {} by {}", title, director),
            Media::AudioBook{title} => format!("AudioBook: {}", title),
            Media::Placeholder => "Placeholder".to_string(),
        }
    }
}

fn print_media(media: &Media) {
    match media {
        Media::Book{title, author} => println!("Book: {} by {}", title, author),
        Media::Movie{title, director} => println!("Movie: {} by {}", title, director),
        Media::AudioBook{title} => println!("AudioBook: {}", title),
        Media::Placeholder => println!("Placeholder"),
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add_item(&mut self, item: Media) {
        self.items.push(item);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        self.items.get(index)
    }

    fn get_by_index2(&self, index: usize) -> MightAValue {
        match self.items.get(index) {
            Some(media) => MightAValue::Value(media),
            None => MightAValue::NoValue,
        }
    }
}

#[derive(Debug)]
enum MightAValue<'a> {
    Value(&'a Media),
    NoValue,
}

fn main() {
    let book = Media::Book{
        title: "The Hobbit".to_string(),
        author: "J.R.R. Tolkien".to_string(),
    };

    print_media(&book);

    let movie = Media::Movie{
        title: "The Hobbit".to_string(),
        director: "Peter Jackson".to_string(),
    };

    print_media(&movie);

    let mut catalog = Catalog::new();
    catalog.add_item(book);
    catalog.add_item(movie);

    println!("{:?}", catalog);

    println!("{:?}", catalog.items.get(0));

    match catalog.items.get(0) {
        Some(media) => print_media(media),
        None => println!("No media found"),
    }

    let media = catalog.get_by_index(0);
    match media {
        Some(media) => print_media(media),
        None => println!("No media found"),
    }

    let media = catalog.get_by_index2(20);
    println!("{:?}", media);

    match media {
        MightAValue::Value(media) => print_media(media),
        MightAValue::NoValue => println!("No media found"),
    }
}
