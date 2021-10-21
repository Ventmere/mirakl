extern crate chrono;
extern crate dotenv;
extern crate mirakl;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;

mod helpers;
use mirakl::result::MiraklError;

macro_rules! dispatch {
  ($matches:expr => $head:tt $($rest:tt)*) => {
    dispatch!(ITEM $matches, $head);
    dispatch!($matches => $($rest)*);
  };

  ($matches:expr => ) => {};

  (ITEM $matches:expr, ($handler:expr)) => {
    ($handler as fn(&clap::ArgMatches))(&$matches)
  };

  (ITEM $matches:expr, ($cmd:ident => $($sub:tt)+)) => {
    if let Some(matches) = $matches.subcommand_matches(stringify!($cmd)) {
      dispatch!(matches => $($sub)*);
    }
  };
}

fn main() {
  ::dotenv::dotenv().unwrap();

  let matches = clap_app!(myapp =>
    (@subcommand order =>
      (about: "Manage orders")
      (@subcommand list_orders =>
      )
      (@subcommand list_pending =>
      )
      (@subcommand test_orders =>
        (@arg FILE: +required "JSON file contains an order array.")
      )
      (@subcommand inspect_order =>
        (about: "Display order items and statuses")
        (@arg ORDER_ID: +required "Mirakl order id")
      )
      (@subcommand accept_line =>
        (about: "Accept order line")
        (@arg ORDER_ID: +required "Mirakl order id")
        (@arg LINE_ID: +required "Mirakl order line id")
      )
      (@subcommand list_order_documents =>
        (about: "List order documents")
        (@arg ORDER_ID: +required "Mirakl order id")
      )
      (@subcommand download_order_document =>
        (about: "Download order document")
        (@arg ORDER_DOCUMENT_ID: +required "Mirakl order document id")
        (@arg OUT: +required "Output file name")
      )
      (@subcommand ship =>
        (about: "ship order")
        (@arg ORDER_ID: +required "Sets the order id")
        (@arg carrier_code: -c --carrier_code +takes_value "Sets the carrier code")
        (@arg tracking_number: -t --tracking_number +takes_value "Sets the tracking number")
        (@arg tracking_url: -u --tracking_url +takes_value "Sets the tracking url")
      )
    )
    (@subcommand offer =>
      (about: "Manage offers")
      (@subcommand dump =>
      )
      (@subcommand deserialize_all =>
        (@arg FILE: +required "File path")
      )
    )
  )
  .get_matches();

  dispatch! {
    matches =>
      (order =>
        (list =>
          (|_| {
            use mirakl::order::*;
            use chrono::{Utc, Duration};
            let client = helpers::get_client();
            let mut params = ListOrdersParams::default();
            params.start_date = Some(Utc::now() - Duration::days(7));
            let orders = {
              match client.list_orders(
                &params,
                None,
                None,
              ) {
                Ok(orders) => orders,
                Err(MiraklError::Deserialize { msg, body }) => {
                  use std::fs::write;
                  write("body.json", body).unwrap();
                  panic!("request error {}, body saved.", msg);
                },
                Err(err) => panic!("{}", err)
              }
            };
            helpers::dump_json(orders)
          })
        )

        (list_pending =>
          (|_| {
            use mirakl::order::*;
            let client = helpers::get_client();
            let mut params = ListOrdersParams::default();
            params.order_state_codes = Some(vec![OrderState::WaitingAcceptance]);
            let res = {
              match client.list_orders(
                &params,
                None,
                None,
              ) {
                Ok(orders) => orders,
                Err(MiraklError::Deserialize { msg, body }) => {
                  use std::fs::write;
                  write("body.json", body).unwrap();
                  panic!("request error {}, body saved.", msg);
                },
                Err(err) => panic!("{}", err)
              }
            };
            println!("pending orders = {}", res.orders.len());
            for order in res.orders {
              helpers::inspect_order(order);
            }
          })
        )

        (test_orders =>
          (|m| {
            use std::fs::{self, File};
            use serde_json::Value;
            let path = m.value_of("FILE").unwrap();

            println!("Loading json file: {}", path);

            let file = File::open(path).unwrap();
            let items: Vec<Value> = serde_json::from_reader(file).unwrap();

            println!("Items: {}", items.len());

            for (i, item) in items.into_iter().enumerate() {
              let text = serde_json::to_string_pretty(&item).unwrap();
              fs::write("last_order.json", &text).unwrap();

              println!("Testing {}...", i);
              serde_json::from_str::<::mirakl::order::Order>(&text).unwrap();
            }

            println!("OK.");

            fs::remove_file("last_order.json").unwrap();
          })
        )

        (inspect_order =>
          (|m| {
            use mirakl::order::*;
            let order_id = m.value_of("ORDER_ID").unwrap();
            let client = helpers::get_client();
            let mut params = ListOrdersParams::default();
            params.order_ids = Some(vec![order_id.to_string()]);
            helpers::inspect_order(client.list_orders(
              &params,
              None,
              None,
            ).unwrap().orders.pop().unwrap())
          })
        )

        (accept_line =>
          (|m| {
            use mirakl::order::*;
            let order_id = m.value_of("ORDER_ID").unwrap();
            let line_id = m.value_of("LINE_ID").unwrap();
            let client = helpers::get_client();
            let accept = OrderAccept {
              order_lines: vec![
                OrderAcceptLine {
                  accepted: true,
                  id: line_id.to_string(),
                }
              ]
            };
            client.accept(
              order_id,
              &accept
            ).unwrap()
          })
        )

        (list_order_documents =>
          (|m| {
            use mirakl::order::document::*;
            let order_id = m.value_of("ORDER_ID").unwrap();
            let client = helpers::get_client();
            let mut params = ListOrderDocumentsParams::default();
            params.order_ids = Some(vec![order_id.to_string()]);
            helpers::dump_json(client.list_order_documents(
              &params
            ).unwrap())
          })
        )

        (download_order_document =>
          (|m| {
            use mirakl::order::document::*;
            use std::fs::File;
            let doc_id = m.value_of("ORDER_DOCUMENT_ID").unwrap();
            let out_name = m.value_of("OUT").unwrap();
            let client = helpers::get_client();
            let mut params = DownloadOrderDocumentsParams::default();
            params.document_ids = Some(vec![doc_id.parse().unwrap()]);
            let out = File::create(out_name).unwrap();
            client.download_order_documents(
              &params,
              out
            ).unwrap();
          })
        )

        (ship =>
          (|m| {
            use mirakl::order::*;
            let order_id = m.value_of("ORDER_ID").unwrap();
            let client = helpers::get_client();
            let carrier_code = m.value_of("carrier_code").and_then(|code| {
              match code.trim().to_lowercase().as_ref() {
                "canada post" => Some("CPCL".to_string()),
                "purolator" => Some("PRLA".to_string()),
                "ups" => Some("UPSN".to_string()),
                "fedex" => Some("FEDX".to_string()),
                "dhl" => Some("DHL".to_string()),
                _ => None,
              }
            });
            let (carrier_name, carrier_url) = if let None = carrier_code.as_ref() {
              (
                m.value_of("carrier_code").map(ToString::to_string),
                m.value_of("tracking_url").map(ToString::to_string)
              )
            } else {
              (None, None)
            };
            let t = OrderTracking {
              carrier_code,
              carrier_name,
              carrier_url,
              tracking_number: m.value_of("tracking_number").map(ToString::to_string),
            };
            client.update_tracking(&order_id, &t).expect("update_tracking");
            client.ship(&order_id).expect("ship");
          })
        )
      )
      (offer =>
        (dump =>
          (|_| {
            use mirakl::client::Method;
            use serde_json::{self, Value};
            let client = helpers::get_client();

            let mut items = vec![];
            loop {
              let res = client.request(Method::GET, &format!(
                 "/api/offers?max=100&offset={}", items.len()
              )).send().unwrap();
              let value: serde_json::Value = res.json().unwrap();
              let mut page_items: Vec<Value> = value.as_object()
                .and_then(|v| v.get("offers"))
                .and_then(|v| v.as_array())
                .unwrap()
                .clone();
              if page_items.is_empty() {
                break
              }
              items.append(&mut page_items);
            }
            serde_json::to_writer_pretty(::std::io::stdout(), &items).unwrap();
          })
        )

        (deserialize_all =>
          (|m| {
            use mirakl::offer::Offer;
            use std::fs::File;
            use serde_json::{self, Value};
            let path = m.value_of("FILE").unwrap();
            let file = File::open(path).unwrap();
            let values: Vec<Value> = serde_json::from_reader(file).unwrap();
            let len = values.len();
            for (i, v) in values.into_iter().enumerate() {
              println!("testing {} of {}...", i + 1, len);
              serde_json::from_value::<Offer>(v.clone()).unwrap();
            }
          })
        )
      )
  }
}
