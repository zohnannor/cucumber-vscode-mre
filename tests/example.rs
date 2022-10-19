use cucumber::{then, when, World};

#[when(regex = "this line matches")]
fn this_line_matches(_: &mut MyWorld) {}

#[then(regex = "this line \
                should too")]
fn this_line_should_too(_: &mut MyWorld) {}

#[derive(Debug, Default, cucumber::World)]
struct MyWorld;

fn main() {
    futures::executor::block_on(MyWorld::run("tests/features"));
}
