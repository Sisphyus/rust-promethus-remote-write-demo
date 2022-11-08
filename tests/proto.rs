include!("../src/server/mod.rs");
use crate::protos::remote::WriteRequest;
use crate::protos::types::{TimeSeries, Sample};
use protobuf::{Message, RepeatedField};

#[test]
fn write_request_test(){
    let mut out_msg = WriteRequest::new();
    let mut ts = TimeSeries::new();
    let mut sample = Sample::new();

    sample.set_timestamp(123);
    sample.set_value(456.0);


    ts.set_samples(RepeatedField::from_vec(vec![sample]));
    out_msg.set_timeseries(RepeatedField::from_vec(vec![ts]));

    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();

    let in_msg: WriteRequest = Message::parse_from_bytes(&out_bytes).unwrap();

    assert_eq!(in_msg.get_timeseries()[0].get_samples()[0].get_timestamp(), 123);
}