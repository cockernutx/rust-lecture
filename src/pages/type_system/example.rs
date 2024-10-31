use std::fmt::Display;

// Define a trait `Media` with a method for getting media type and content.
trait Media {
    fn media_type(&self) -> &str;
    fn content(&self) -> &str;
}

// Implement `Media` for a `Text` struct.
struct Text<'a> {
    content: &'a str,
}

impl<'a> Media for Text<'a> {
    fn media_type(&self) -> &str {
        "Text"
    }

    fn content(&self) -> &str {
        self.content
    }
}

// Implement `Media` for an `Audio` struct.
struct Audio<'a> {
    content: &'a str,
}

impl<'a> Media for Audio<'a> {
    fn media_type(&self) -> &str {
        "Audio"
    }

    fn content(&self) -> &str {
        self.content
    }
}

// Implement `Media` for a `Video` struct.
struct Video<'a> {
    content: &'a str,
}

impl<'a> Media for Video<'a> {
    fn media_type(&self) -> &str {
        "Video"
    }

    fn content(&self) -> &str {
        self.content
    }
}

// A generic `MediaLibrary` that can hold any type implementing `Media`.
struct MediaLibrary<'a, T: Media> {
    media_items: Vec<&'a T>,
}

impl<'a, T: Media> MediaLibrary<'a, T> {
    fn new() -> Self {
        MediaLibrary {
            media_items: Vec::new(),
        }
    }

    fn add_media(&mut self, item: &'a T) {
        self.media_items.push(item);
    }

    fn display_all(&self)
    where
        T: Display,
    {
        for item in &self.media_items {
            println!("{}: {}", item.media_type(), item.content());
        }
    }
}

// Implement `Display` for each media type to satisfy the `display_all` constraint.
impl<'a> Display for Text<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<'a> Display for Audio<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<'a> Display for Video<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

// Example usage
#[allow(dead_code)]
fn create_library() {
    let text = Text {
        content: "Hello, world!",
    };
    let audio = Audio {
        content: "audio_file.mp3",
    };
    let video = Video {
        content: "video_file.mp4",
    };

    // Create separate libraries for each type
    let mut text_library = MediaLibrary::new();
    text_library.add_media(&text);

    let mut audio_library = MediaLibrary::new();
    audio_library.add_media(&audio);

    let mut video_library = MediaLibrary::new();
    video_library.add_media(&video);

    println!("Text Library:");
    text_library.display_all();

    println!("\nAudio Library:");
    audio_library.display_all();

    println!("\nVideo Library:");
    video_library.display_all();
}
