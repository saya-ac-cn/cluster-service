use std::fmt::Display;

struct Compare<T> {
    param1:T,
    param2:T
}

impl <T> Compare<T>{
    fn builder(param1:T,param2:T) -> Self{
        Self{param1,param2}
    }
}

impl <T> Compare<T> where T:Display + PartialOrd {
    pub fn compare(&self){
        if self.param1 == self.param2 {
            println!("等于");
        } else if self.param1 > self.param2 {
            println!("大于");
        }else {
            println!("小于");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::compare::Compare;

    #[test]
    fn run(){
        let comparator = Compare{ param1:12,param2: 13 };
        comparator.compare();
    }
}
