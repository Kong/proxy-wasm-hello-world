// from https://github.com/rtyler/xmltojson
extern crate serde_json;

use log::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct Error {}

fn read(mut reader: &mut Reader<&[u8]>, depth: u64, parse_attributes: bool) -> Value {
    let mut buf = Vec::new();
    let mut values = Vec::new();
    let mut node = Map::new();
    debug!("Parsing at depth: {}", depth);

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if let Ok(name) = String::from_utf8(e.name().to_vec()) {
                    let child = read(&mut reader, depth + 1, parse_attributes);
                    let new_name = name.split(':').rev().next().unwrap().to_string();

                    debug!("{} children: {:?}", new_name, child);

                    if node.contains_key(&new_name) {
                        debug!("Node contains `{}` already, need to convert to array", new_name);
                        let (_, mut existing) = node.remove_entry(&new_name).unwrap();
                        let mut entries: Vec<Value> = vec![];

                        if existing.is_array() {
                            let existing = existing.as_array_mut().unwrap();
                            while !existing.is_empty() {
                                entries.push(existing.remove(0));
                            }
                        } else {
                            entries.push(existing);
                        }
                        entries.push(child);

                        node.insert(new_name, Value::Array(entries));
                    } else {
                        node.insert(new_name, child);
                    }
                }
            }
            Ok(Event::Text(ref e)) => {
                if let Ok(decoded) = e.unescape_and_decode(&reader) {
                    values.push(Value::String(decoded));
                }
            }
            Ok(Event::CData(ref e)) => {
                if let Ok(decoded) = e.unescape_and_decode(&reader) {
                    node.insert("#cdata".to_string(), Value::String(decoded));
                }
            }
            Ok(Event::End(ref _e)) => break,
            Ok(Event::Eof) => break,
            _ => (),
        }
    }

    debug!("values to return: {:?}", values);
    if !node.is_empty() {
        // If we had collected some text along the way, that needs to be inserted
        // so we don't lose it
        let mut index = 0;
        let mut has_text = false;
        for value in values.iter() {
            if value.is_string() {
                has_text = true;
                break;
            }
            index += 1;
        }

        if has_text {
            node.insert("#text".to_string(), values.remove(index));
        }
        debug!("returning node instead: {:?}", node);
        return serde_json::to_value(&node).expect("Failed to #to_value() a node!");
    }

    match values.len() {
        0 => Value::Null,
        1 => values.pop().unwrap(),
        _ => Value::Array(values),
    }
}

pub fn to_json(xml: &str, parse_attributes: bool) -> Result<Value, Error> {
    let mut reader = Reader::from_str(xml);
    reader.expand_empty_elements(true);
    reader.trim_text(true);

    Ok(read(&mut reader, 0, parse_attributes))
}

pub fn go_to_body(root: &Value) -> Option<&Value> {
    let envelop = root.get("Envelope").unwrap();

    envelop.get("Body")
}
