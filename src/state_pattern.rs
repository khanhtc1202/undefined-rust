pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // We call the as_ref method on the Option because we want a reference to the value inside the Option
        // rather than ownership of the value.
        // Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned.
        // If we didn’t call as_ref, we would get an error because we can’t move state out of the borrowed &self of the function parameter.
        //
        // At this point, when we call content on the &Box<dyn State>, deref coercion will take effect on the & and the Box
        // so the content method will ultimately be called on the type that implements the State trait.
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        // We need to set state to None temporarily rather than setting it directly with code like
        // self.state = self.state.request_review(); to get ownership of the state value.
        // This ensures Post can’t use the old state value after we’ve transformed it into a new state.
        //
        // aka. take takes out the value State from Post and it temp assign to None, then after the
        // request operation done, it been assign to the new value of State implemented struct
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub fn run() {
    let mut post = Post::new();

    post.add_text("I do something here");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I do something here", post.content());
}