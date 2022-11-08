use flume::Sender;
use super::protos::remote::WriteRequest;
pub struct AppState {
    pub channel: Sender<WriteRequest>,
}

impl AppState {
    pub fn new(sender: Sender<WriteRequest>) -> AppState {
        AppState{
            channel: sender
        }
    }
}