use cucumber::{when, World};

#[when(regex = "test")]
fn test(_: &mut MyWorld) {}

#[derive(Debug, Default, cucumber::World)]
struct MyWorld;

fn main() {
    futures::executor::block_on(MyWorld::run("tests/features"));
}
