pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

#[derive(Debug)]
pub enum linked_list<T> {
    Cons(T, Box<linked_list<T>>),
    Nil,
}

pub fn run() {
    let list: linked_list<i32> =
        linked_list::Cons(1, Box::new(linked_list::Cons(2, Box::new(linked_list::Nil))));
    println!("{:?}", list);
}
