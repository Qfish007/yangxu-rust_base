fn main() {
    println!("Example p54: OOP 状态模式");
    test01();
}

pub(crate) trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // 创建一个草稿状态的帖子，默认是草稿状态
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(DraftState {})),
            content: String::new(),
        }
    }

    // 添加内容到帖子
    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    // 获取帖子的内容
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // 请求审核帖子
    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    // 通过审核帖子
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

struct DraftState {}

impl State for DraftState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingState {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingState {}

impl State for PendingState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PublishedState {})
    }
}

struct PublishedState {}

impl State for PublishedState {
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

#[allow(unused)]
fn test01() {
    let mut post = Post::new();
    post.add_content("hello world");
    post.request_review();
    post.approve();
    println!("{}", post.content());
}
