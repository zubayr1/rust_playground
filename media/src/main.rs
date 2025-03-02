pub mod content;

use content::*;

fn print_media(media: &Media) {
    match media {
        Media::Book{title, author} => println!("Book: {} by {}", title, author),
        Media::Movie{title, director} => println!("Movie: {} by {}", title, director),
        Media::AudioBook{title} => println!("AudioBook: {}", title),
        Media::Placeholder => println!("Placeholder"),
    }
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
