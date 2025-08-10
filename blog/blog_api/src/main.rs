use blog_shared::Post;
fn main() {
    let blog = Post::new("Hello world".to_string(), "This is a test body".to_string());

    println!("Result: {:?}", blog.as_json())
}
