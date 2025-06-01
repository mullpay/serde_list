#[derive(serde::Serialize)]
struct Foo {
    b: bool,
}

#[derive(serde::Serialize)]
struct Bar {
    a: i32,
    b: i32,
}

#[derive(serde_list::AsList)]
#[tag = "my_tag"]
struct J {
    foo: Foo,
    bar: Bar,
}

fn main() {
    let j = J {
        foo: Foo { b: false },
        bar: Bar { a: 10, b: 20 },
    };
    let value = serde_json::to_string_pretty(&j).unwrap();
    println!("{value}");
}
