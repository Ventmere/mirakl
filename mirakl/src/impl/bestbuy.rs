use super::MiraklImpl;

pub struct Bestbuy;

impl MiraklImpl for Bestbuy {
  const ENDPOINT: &'static str = "https://marketplace.bestbuy.ca";
}
