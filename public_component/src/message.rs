pub struct Message{
    pub name:String,
    pub mail:String,
    pub title:String,
    pub content:String,
    pub time:String
}

impl Message {
    pub fn head(&self) -> String {
       format!("\n收件人：{}\n邮件地址：{}\n邮件标题：{}",self.name,self.mail,self.title)
    }
}

impl Message {
    pub fn body(&self) -> String {
        format!("\n邮件正文：{}",self.content)
    }
}


impl Message {
    pub fn footer(&self) -> String {
        format!("\n发送时间：{}",self.time)
    }
}
#[cfg(test)]
mod test {
    use crate::message;

    #[test]
    fn run(){
        let e_mail = message::Message{
            name: String::from("亲亲里"),
            mail: String::from("saya@saya.ac.cn"),
            title: String::from("年度运营统计"),
            content: String::from("您好。平台已为您生成年度运营报告，赶紧去看看吧~"),
            time: String::from("2023-01-12 11:01:52")
        };
        println!("{}",e_mail.head());
        println!("{}",e_mail.body());
        println!("{}",e_mail.footer());
    }
}
