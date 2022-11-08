use std::collections::HashMap;

use super::{handler, state::AppState, ts};
use super::{protos::remote::WriteRequest, protos::types::TimeSeries};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use flume::{Receiver, Sender};
use log;
use protobuf::RepeatedField;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub struct Server {
    pub channel: (Sender<WriteRequest>, Receiver<WriteRequest>),
    pool: ThreadPool,
    batch_size: usize,
}

impl Server {
    pub fn new(cap: usize, num_threads: usize) -> Server {
        Server {
            channel: flume::bounded::<WriteRequest>(cap),
            pool: ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .unwrap(),
            batch_size: 5,
        }
    }

    async fn consume(&self) {
        log::info!("start consume");
        let mut temp_vec = Vec::new();
        while let Ok(wr) = self.channel.1.recv_async().await {
            log::info!("Received ts, length: {:?}", wr.get_timeseries().len());
            log::info!("current batch size: {}", temp_vec.len());
            if temp_vec.len() < self.batch_size {
                temp_vec.push(wr)
            } else {
                self.pool.install(|| self.map(&temp_vec));
                temp_vec.clear()
            }
        }
        log::info!("end consume");
    }

    fn map(&self, wrs: &[WriteRequest]) {
        let mut m: HashMap<String, Vec<TimeSeries>> = HashMap::new();
        for wr in wrs {
            for ts in wr.get_timeseries() {
                let key_temp = ts::get_unique_key(ts.get_labels());
                let key = key_temp.as_str();
    
                for sample in ts.get_samples() {
                    let key2 = ts::get_unique_key_v2(key, sample.get_timestamp());
    
                    let mut new_ts = TimeSeries::new();
                    new_ts.set_labels(RepeatedField::from_vec(ts.get_labels().to_vec()));
                    new_ts.set_samples(RepeatedField::from_vec(vec![sample.clone()]));
    
                    let value = m.get_mut(&key2);
                    match value {
                        Some(v) => v.push(new_ts),
                        None => {
                            m.insert(key2, vec![new_ts]);
                        }
                    }
                }
            }
        }

        for (key, tss) in m.iter() {
            log::info!("get unique key: {}", key);
            log::info!("get tss length: {}", tss.len());
        }
        return;
    }

    pub async fn run(self) {
        let sender = self.channel.0.clone();
        let consumer = self.consume();
        let (_a, _b) = futures::join!(consumer, launch(sender));

        // let launch = actix_web::rt::spawn(launch(sender));
        // let consumer = actix_web::rt::spawn(self.consume());
        // launch.await;
        // consumer.await;
    }
}

pub async fn launch(sender: Sender<WriteRequest>) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState::new(sender.clone())))
            .service(handler::echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
