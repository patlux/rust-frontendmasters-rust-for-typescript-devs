fn foo(nr: usize) {
    println!("foo({})", nr);
    foo(nr + 1)
}

fn main() {
    foo(0);
}

#[cfg(test)]
mod Tests {}
