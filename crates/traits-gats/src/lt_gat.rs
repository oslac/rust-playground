//! # Lifetime GATs
//!
//! Lifetime GATs allow the declaration of a generic lifetime on the associated
//! type, for when we need to tie the lifetime of some associated type to a
//! lifetime in the trait method.
//!
//! (An associated type is just a *type alias with a purpose* as documented in
//! the given trait interface).
//!
//! ## Motivation
//!
//! Without lifetime GATs, we cannot specify the lifetime requirements in trait
//! methods or associated types for **borrowed types**.
//!
//! ### Example
//!
//! Here the concrete type `Test` has some lifetimes of its own, that we
//! would like to refer to in a trait. Without lifetime GATs, they cannot be
//! referred to in the associated type in the actual `impl`.
//!
//! ```ignore
//! pub struct Test<'a>(&'a str);
//! ```
//!
//! This trait `T` is not using a lifetime GAT, so in the impl it cannot take in
//! `Test` as the `type Input`, because it cannot speak about the lifetime
//! requirements / bounds, iow. **borrowed types** cannot be used as the
//! associated types without lifetime GATs.
//!
//! ```ignore
//! trait T {
//!     // ! No lifetime specifier without lifetime GATs !
//!     type Input;
//!     fn handle(&self, t: Self::Input);
//! }
//! ```
//!
//! In attempt to make the impl, we cannot make the associated type `Input` to
//! take in  the `Test` struct, because there is no way to refer to the
//! lifetime(s):
//!
//! ```ignore
//! struct ImplT {}
//!
//! impl T for ImplT {
//!     // Can't refer to the lifetime required by `Test`, causing errors.
//!     type Input = Test;
//!
//!     fn handle(&self, t: Self::Input) {
//!         todo!()
//!     }
//! }
//! ```

/// An actual trait using lifetime GATs, enabling the assignment of a **borrowed
/// type** as the [T::Input].
pub trait T {
    /// Lifetime `'a` is generic over *any lifetime*.
    type Input<'a>;

    /// "For whatever lifetime, `input` doesn't live beyond
    /// [handle](T::handle)".
    fn handle(&self, input: Self::Input<'_>);
}

/// Some other struct implementing the trait [T](T) for demonstration purposes.
pub struct ImplT;

impl T for ImplT {
    /// It is now possible to name the lifetime for `Test` when using lifetime
    /// GATs:
    type Input<'s> = Test<'s>;

    /// Note that `handle` takes the input by value. In this case, since
    /// references are first class values in Rust, their own types, we can take
    /// the **borrowed type by value** when using lifetime GAT.
    fn handle(&self, input: Self::Input<'_>) {
        print!("{}", input.0)
    }
}

/// Test structure with its own lifetime for demonstration purposes.
#[derive(Debug)]
pub struct Test<'a>(&'a str);

/// Usage example.
pub fn example() {
    let data = "text";
    let test = Test(data);

    let gat = ImplT {};
    let _call = gat.handle(test);

    // Case 1: `test` variable.
    //
    // Error: borrow of a moved value.
    //
    // The borrowed type is taken by value (iow. moved).
    // `test` doesn't live beyond the call to `handle` as desired.
    println!("{:#?}", test);

    // Case 2: `data` variable.
    //
    // No error.
    //
    // This data is borrowed by `Test` for `'_` lifetime. It must live at least as
    // long as `Test` does, or longer, so it doesn't turn into dangling pointer.
    println!("{}", data);
}
