use oo_design_pattern::{Post, PendingReviewPost, DraftPost};

fn main() {
    let mut post: DraftPost = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post: PendingReviewPost = post.request_review();

    let post: Post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
