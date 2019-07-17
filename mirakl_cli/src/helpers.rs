use mirakl::{client::MiraklClient, order::*, MiraklImpl};
use serde::Serialize;
use serde_json;
use std::env::var;
use std::io::stdout;

pub fn get_client() -> MiraklClient {
  let token = var("TOKEN").unwrap();

  match var("IMPL").unwrap().as_ref() {
    "CatchComAu" => MiraklClient::new(MiraklImpl::CatchComAu, &token),
    "BestbuyCa" => MiraklClient::new(MiraklImpl::BestbuyCa, &token),
    v => panic!("unknown impl '{}'", v),
  }
}

pub fn dump_json<T: Serialize>(v: T) {
  serde_json::to_writer_pretty(stdout(), &v).unwrap()
}

pub fn inspect_order(order: Order) {
  println!("id: {}", order.order_id);
  println!("status: {:?}", order.order_state);
  println!("lines:\n");
  for line in order.order_lines {
    println!(
      "\t#{}\t{:?}\t{}\t{}\t{}",
      line.order_line_id, line.order_line_state, line.offer_sku, line.price_unit, line.quantity
    );
  }
}
