fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    // default
    let default_float = 3.0;
    let default_integer = 7;

    // type can also be inferred from context
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    // Variables can be overwritten with shadowing.
    let mutable = true;
}
