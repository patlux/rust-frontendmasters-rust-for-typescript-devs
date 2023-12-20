struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::Number(2))
}

fn main() {
    let foo = Item::MyCustom(Custom {
        age: 30,
        name: "Patrick".into(),
    });

    let mut items: Vec<_> = vec![
        Item::Number(2),
        Item::String("Patrick".into()),
        Item::MyCustom(Custom {
            age: 30,
            name: "Patrick Wozniak".into(),
        }),
    ];

    append(&mut items);
}
