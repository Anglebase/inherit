# #[inherits(Type)]

This attribute macro achieves inheritance-like effects through the `Deref` and `DerefMut` traits, simplifying the application of the composition pattern in Rust.

A struct annotated with `#[inherit(T)]` will implicitly contain a `parent` field of type `T`, enabling single inheritance behavior.

For disambiguated access to fields with the same name, use `(&<name> as &<type>).<field>` or `(&mut <name> as &mut <type>).<field>` syntax.