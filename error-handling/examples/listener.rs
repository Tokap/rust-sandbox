// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate kafka;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;

mod errors {
  error_chain! {
    foreign_links {
      KafkaError(::kafka::error::Error);
    }
  }
}

mod domain;
mod lib;

use errors::*;
use kafka::consumer::Consumer;
use kafka::producer::{Producer, Record};


fn main() {
  if let Err(ref e) = run() {
    println!("error: {}", e);

    for e in e.iter().skip(1) {
        println!("caused by: {}", e);
    }

    if let Some(backtrace) = e.backtrace() {
        println!("backtrace: {:?}", backtrace);
    }

    ::std::process::exit(1);
  }
}


fn run() -> Result<()> {

  // mysql
  let db_url = "mysql://root@localhost/ip_hoarder";
  let pool   = mysql::Pool::new(db_url).unwrap();

  // kafka consumer
  let broker    = "localhost:9092";
  let topic     = String::from("test");
  let group     = String::from("social-hoarder-consumers");
  let kafka_con = lib::kafka::get_consumer(group, topic, vec![broker.to_string()])?;
  let kafka_pro = lib::kafka::get_producer(vec![broker.to_string()])?;

  // start listener
  if let Err(e) = listener(pool, kafka_con, kafka_pro) {
    println!("Failed consuming messages: {}", e);
  };

  Ok(())
}


fn listener(my_pool: mysql::Pool,
            mut kafka_con: Consumer,
            mut kafka_pro: Producer)
            -> Result<()> {

  // produce message
  let value = "{ butts\"command\":\"social_hoarder:add_user\", \"correlation_id\":\"7395671238\", \"payload\":{\"network\":\"instagram\",\"user_id\":\"123\",\"username\":\"chaileeson\"}}".as_bytes();
  kafka_pro.send(&Record::from_value("test", value))?;


  // consume message
  loop {
    // consume messagesets from kafka
    let mss = kafka_con.poll()?;

    for ms in mss.iter() {

      for m in ms.messages() {

        // parse kafka message into KafkaCommand
        let new_kafka_message = lib::kafka::parse_fetched_message(m)?;

        // instantiate network_account
        let new_network_account = domain::network_account::NetworkAccount {
          id       : None,
          network  : new_kafka_message.payload.network.to_owned(),
          user_id  : new_kafka_message.payload.user_id.to_owned(),
          username : new_kafka_message.payload.username.to_owned(),
        };

        // insert network_account record into mysql
        domain::network_account::insert(&my_pool, new_network_account)?;
      }

      let _ = kafka_con.consume_messageset(ms);
    }

    try!(kafka_con.commit_consumed());
  };
}
