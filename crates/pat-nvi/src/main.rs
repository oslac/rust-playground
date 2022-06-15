use pat_nvi::nvi::{nvi_impl, Database};

fn main() {
    println!("NVI Interface");
    let mut persistence = nvi_impl::new();
    let _ = persistence.insert(0, "A".to_string());
    let load_a = load_item(&persistence, 0);

    println!("{:#?}", load_a)
}

// Example: no more traits in arguments. This preserves polymorphism because it
// still stores the impl, but it also reduces amount of generic code because its
// a concrete type.
fn load_item(db: &Database, id: usize) -> Option<String> {
    db.load(id)
}
