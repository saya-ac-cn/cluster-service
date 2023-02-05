use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};

/// 登录校验
#[derive(Clone)]
pub enum LoginCheckEnum {
    // 密码
    Password,
    // 图片验证
    Image,
    // 短信
    Message,
}

impl From<LoginCheckEnum> for &str {
    fn from(arg: LoginCheckEnum) -> Self {
        match arg {
            LoginCheckEnum::Password => "password",
            LoginCheckEnum::Image => "image",
            LoginCheckEnum::Message => "message",
        }
    }
}

impl From<&str> for LoginCheckEnum {
    fn from(arg: &str) -> Self {
        match arg {
            "password" => LoginCheckEnum::Password,
            "image" => LoginCheckEnum::Image,
            "message" => LoginCheckEnum::Message,
            _ => LoginCheckEnum::Password,
        }
    }
}

impl Debug for LoginCheckEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&str>::from(self.clone()))
    }
}

impl Display for LoginCheckEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&str>::from(self.clone()))
    }
}

impl serde::Serialize for LoginCheckEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for LoginCheckEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = String::deserialize(deserializer)?;
        Ok(LoginCheckEnum::from(v.as_str()))
    }
}
