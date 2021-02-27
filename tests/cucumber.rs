use cucumber_rust::{async_trait, given, then, when, World, WorldInit};
use std::convert::Infallible;
use tempfile::{tempdir, TempDir};

/// Mutable context for scenarios.
#[derive(WorldInit)]
pub struct MyWorld {
  /// full path to the folder containing the workspace for this scenario
  workspace: TempDir,
}

impl MyWorld {
  // async fn test_async_fn(&mut self) {
  //   *self.some_value.borrow_mut() = 123u8;
  //   self.bar = 123;
  // }
}

#[async_trait(?Send)]
impl World for MyWorld {
  type Error = Infallible;

  async fn new() -> Result<Self, Infallible> {
    Ok(Self {
      workspace: tempdir().unwrap(),
    })
  }
}

impl Drop for MyWorld {
  fn drop(&mut self) {
    println!("Dropping MyWorld!");
  }
}

#[given(regex = "^the workspace contains file \"(.*)\" with content:$")]
fn the_workspace_contains_file_with_content(world: &mut MyWorld, filename: String) {
  assert_eq!(filename, "implement");
}

// #[given("a thing")]
// async fn a_thing(world: &mut MyWorld) {
//   world.foo = "elho".into();
//   world.test_async_fn().await;
// }

// #[when(regex = "something goes (.*)")]
// async fn something_goes(_: &mut MyWorld, _wrong: String) {}

// #[given("I am trying out Cucumber")]
// fn i_am_trying_out(world: &mut MyWorld) {
//   world.foo = "Some string".to_string();
// }

// #[when("I consider what I am doing")]
// fn i_consider(world: &mut MyWorld) {
//   let new_string = format!("{}.", &world.foo);
//   world.foo = new_string;
// }

// #[then("I am interested in ATDD")]
// fn i_am_interested(world: &mut MyWorld) {
//   assert_eq!(world.foo, "Some string.");
// }

// #[then(regex = "^we can (.*) rules with regex$")]
// fn we_can_regex(_: &mut MyWorld, action: String) {
//   // `action` can be anything implementing `FromStr`.
//   assert_eq!(action, "implement");
// }

#[tokio::main]
async fn main() {
  let runner = MyWorld::init(&["./features"]);
  runner.run_and_exit().await;
}
