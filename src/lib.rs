//! # Hecs-schedule
//!
//! hecs-schedule is a framework for [hecs](https://crates.io/crates/hecs) that
//! provides system abstractions for paralell execution.
//!
//! ## SubWorld
//!
//! [SubWorld]( crate::SubWorld ) provides the ability to split the world into smaller parts
//! which can only access a subset of components. This allows
//!
//! ## Commandbuffer
//!
//! [CommandBuffer]( crate::CommandBuffer ) provides deferred world modification by
//! means of component insertion, removal, entity spawning and despawning, as well
//! as arbitrary world modification by closures, which will be executed at a later
//! time.
//!
//! The commandbuffer extends the already existing hecs::CommandBuffer and provides
//! more functionality.
//!
//! ## System and Schedule
//!
//! A system represents a unit of work which can access any resource. Systems are
//! implemented for any function and closure with any number of arguments (well, up
//! to a sane limit due to tuple size and compile time).
//!
//! A system may access a subworld and safely access the declared components. It can
//! also access any other value by type with [Read](crate::Read) and [Write](crate::Write) wrappers.
//!
//! This value will be pulled from the provided [Context](crate::Context) which is
//! provided to [Schedule::execute] as a mutable reference. This means that systems
//! can access local variable and struct members from outside the ECS. If a value of
//! the type was not provided, the system will exit cleanly with an error.
//!
//! Systems can either return nothing or an empty result, which will be properly
//! boxed and propogated
//!
//! The schedule is a collection of ordered system executions.
//!
//! When a schedule is executed, a tuple of references for the contained systems
//! will be provided.
//!
//! ## Usage
//!
//! ```rust
//! use hecs_schedule::*;
//! use hecs::*;
//!
//! let mut world = World::default();
//!
//! #[derive(Debug)]
//! struct App {
//!     name: &'static str,
//! }
//!
//! let mut app = App {
//!     name: "hecs-schedule"
//! };
//!
//! // Spawn some entities
//! let a = world.spawn(("a", 42));
//! world.spawn(("b", 0));
//! world.spawn(("c", 7));
//!
//! // Create a simple system to print the entities
//! let print_system = | w: SubWorld<(& &'static str, &i32)> | {
//!   w.query::<(&&'static str, &i32)>().iter().for_each(|(e, val)| {
//!     println!("Entity {:?}: {:?}", e, val);
//!   })
//! };
//!
//! // Get a component from a specific entity, failing gracefully if the entity
//! // didn't exist or the subworld did not support the component. The result
//! // will propogate to the schedule execution.
//! let get_system = move | w: SubWorld<&i32> | -> anyhow::Result<()> {
//!   let val = w.get::<i32>(a)?;
//!
//!   // Prints the answer to life, the universe, and everything.
//!   // Welp, maybe not how to please the borrow checker, but almost
//!   // everything.
//!   println!("Got: {}", *val);
//!
//!   Ok(())
//! };
//!
//! // Declare a system which borrows the app and prints it.
//! // This requires that a reference to app was provided to execute.
//! // Otherwise, the system fails and returns an error, which propogates to the
//! // schedule and stops execution.
//!
//! // It is also possible to modify the app via `mut Write<App>`
//! let print_app = |app: Read<App>| {
//!     println!("App: {:?}", app);
//! };
//!
//! // Note: the `hecs_schedule::CommandBuffer` is a superset of `hecs::CommandBuffer` and is
//! // accesible as a shared resource from systems.
//! let spawn_system = |mut cmd: Write<hecs_schedule::CommandBuffer>| {
//!     cmd.spawn(("c", 5));
//! };
//!
//! // Construct a schedule
//! let mut schedule = Schedule::builder()
//!     .add_system(spawn_system)
//!     .add_system(print_system)
//!     .add_system(print_app)
//!     .add_system(get_system)
//!     .build();
//!
//! // Execute the schedule's systems and provide the world and app. This will parallelize as much
//! // as possible.
//! schedule.execute((&mut world, &mut app)).expect("Failed to execute schedule");
//!
//! ```

#![warn(missing_docs)]
#[macro_use]
mod macros;
mod access;
#[macro_use]
pub mod borrow;
mod commandbuffer;
pub mod context;
pub mod error;
mod query;
mod schedule;
mod subworld;
mod subworld_impls;
pub mod system;
pub mod traits;

pub use access::*;
pub use borrow::{Read, Write};
pub use commandbuffer::*;
pub use context::*;
pub use error::Error;
pub use query::*;
pub use subworld_impls::*;
// Don't export result so that hecs-schedule can be glob imported without
// conflict
pub(crate) use error::Result;
pub use schedule::*;
pub use subworld::*;
pub use system::*;
