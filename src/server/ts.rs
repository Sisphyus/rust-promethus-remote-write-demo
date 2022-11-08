extern crate concat_string;
use super::protos::types::Label;

const GROUP: &str = "group";
const SDU: &str = "sdu";
const NAME: &str = "__name__";
const CMDB_SERVICE_NAME: &str = "cmdb_service_name";
const DATACENTER: &str = "datacenter";
const KEY_SPLIE_FLAG: &str = "====";


pub fn get_unique_key(labels: &[Label]) -> String {
    let mut str: String = String::new();
    for label in labels {
        if label.name == NAME
            || label.name == CMDB_SERVICE_NAME
            || label.name == SDU
            || label.name == DATACENTER
            || label.name == GROUP
        {
            str = concat_string::concat_string!(str, label.value)
        }
    }
    str
}

pub fn get_unique_key_v2(key: &str, timestamp: i64) -> String {
    concat_string::concat_string!(key, KEY_SPLIE_FLAG, (timestamp/1000).to_string())
}
