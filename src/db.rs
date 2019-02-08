extern crate postgres;
extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

use postgres::{Connection, TlsMode};
use serde_xml_rs::{from_reader};
use std::env::var;

struct DbRow {
    id: i32,
    text: String,
}

#[derive(Deserialize, Debug)]
struct Siri {
    #[serde(rename = "ServiceDelivery")]
    service_delivery: ServiceDelivery,
}

#[derive(Deserialize, Debug)]
struct ServiceDelivery {
    #[serde(rename = "ResponseTimestamp")]
    response_timestamp: String,
    
    #[serde(rename = "ProducerRef")]
    producer_ref: String,
    
    #[serde(rename = "MoreData")]
    more_data: bool,
}

fn get_connection() -> Connection {
    let db_connection_string = var("DATABASE_URI").unwrap();
    return Connection::connect(db_connection_string, TlsMode::None).unwrap();
}

pub fn get_row() {
    let conn = get_connection();
    for row in &conn.query("SELECT id, text FROM busbank_text LIMIT 1", &[]).unwrap() {
        let result = DbRow {
            id: row.get(0),
            text: row.get(1),
        };
        println!("Retrieved row {}", result.id);
        parse_siri_xml(result.text);
    }
}

fn parse_siri_xml(text: String) -> Siri {
    println!("Parsing xml");
    let result = from_reader(text.as_bytes());
    let data: Siri = match result {
        Ok(x) => x,
        Err(error) => {
            panic!("Something went wrong while destructuring transactions response: {}", error);
        }
    };

    println!("XML: {:#?}", data);

    return data;
}
