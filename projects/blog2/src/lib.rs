#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let post = post.approve();
        let mut post = post.reject();
        post.add_text("\nIt was delicious"); // It WILL add because we ARE in draft state
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();

        assert_eq!(
            "I ate a salad for lunch today\nIt was delicious",
            post.content()
        );
    }
}

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> ScheduledPost {
        ScheduledPost {
            content: self.content,
        }
    }
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub struct ScheduledPost {
    content: String,
}

impl ScheduledPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
