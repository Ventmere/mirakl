mod bestbuy;
mod catch_com_au;

pub trait MiraklImpl {
  const ENDPOINT: &'static str;
}

pub use self::bestbuy::Bestbuy;
pub use self::catch_com_au::CatchComAu;
