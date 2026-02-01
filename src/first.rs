// pub enum List {
//     Empty,
//     Elem(i32, List)
// } 

// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }


// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }

// struct Node {
//     elem: i32,
//     next: List,
// }

// pub enum List {
//     Empty,
//     More(Box<Node>),
// }

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

