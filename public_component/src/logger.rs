/// 定义一个日志打印的接口
pub trait Logger {
    fn construct(&self) -> String {
        String::from("(Read more...)")
    }
    fn printer(&self) -> String;
}

/// Debug模式
#[derive(Debug,Clone)]
pub struct Debug {
    pub title: String,
    pub content: String,
}

impl Logger for Debug {
    fn printer(&self) -> String {
        format!("Debug {} {}", self.title, self.content)
    }
}

/// Error模式
#[derive(Debug,Clone)]
pub struct Error {
    pub title: String,
    pub location: String,
    pub content: String,
}

impl Logger for Error {
    fn printer(&self) -> String {
        format!("Error {} cause by:{} detail:{}", self.title, self.content,self.location)
    }
}

pub fn builder(log: &(impl Logger + Clone)){
    println!("trait 作为参数:{}",log.printer())
}

pub fn builder_<T: Logger + Clone>(log: &T){
    println!("Trait Bound:{}",log.printer())
}

pub fn builder_where<T>(log:&T) where T:Logger + Clone{
    println!("Where Trait Bound:{}",log.printer())
}

#[cfg(test)]
mod test {
    use crate::{logger, Logger};

    #[test]
    fn run() {
        let debugger = logger::Debug{title:String::from("primary_server"),content:String::from("Starting 6 workers")};
        println!("调试日志-{}",debugger.printer());

        let error = logger::Error{title:String::from("primary_server"),location:String::from("/log"),content:String::from("Starting 6 workers")};
        println!("错误日志-{}",error.printer());

        logger::builder(&debugger);
        logger::builder_(&debugger);
        logger::builder_where(&debugger)
    }
}
