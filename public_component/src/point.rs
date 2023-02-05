#[cfg(test)]
mod test {
    use crate::point::test::List::{Cons, Nil};

    #[test]
    fn point_15_1(){
        let val = Box::new(5);
        println!("val = {}",val)
    }

    enum List{
        Cons(i32,Box<List>),
        Nil,
    }

    #[test]
    fn point_15_5(){
        let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
    }
}
