use std::net::SocketAddr;

use crate::error::CounchError;
use crate::network::{BroadcastMsg, OnReceive, Peer, SwitcherI};
use log::{debug, error, info, log_enabled, Level};
use warp;
use warp::Filter;

fn log_init() {
    use std::io::Write;
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();
}

pub struct HTTPNetWork {
    pub addrs: Vec<String>,    // 所有的节点地址
    pub peer_ids: Vec<String>, //
    pub local_address: String, // 本机地址
    pub mode_id: String,       //   // 节点ID
}

impl HTTPNetWork {
    pub fn new(
        addrs: Vec<String>,
        peer_ids: Vec<String>,
        local_address: String,
        mode_id: String,
    ) -> Self {
        log_init();
        HTTPNetWork {
            addrs,
            peer_ids,
            local_address,
            mode_id,
        }
    }

    pub async fn start(&self) {
        //todo:: 启动服务
        // POST /employees/:rate  {"name":"Sean","rate":2}
        let promote = warp::post()
            .and(warp::path("employees"))
            .and(warp::path::param::<u32>())
            // Only accept bodies smaller than 16kb...
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|rate, mut employee: Employee| {
                employee.rate = rate;
                warp::reply::json(&employee)
            });
        let addr = self.local_address.parse::<SocketAddr>().unwrap();
        warp::serve(hello).run(addr).await;
    }
}

impl SwitcherI for HTTPNetWork {
    fn broadcast(model_id: String, msg: &BroadcastMsg) -> CounchError {
        //todo::
        CounchError::UnImpl
    }
    fn broadcast_to_peer(model_id: String, msg: &BroadcastMsg, p: &Peer) -> CounchError {
        CounchError::UnImpl
    }
    fn broadcast_except_peer(model_id: String, msg: &BroadcastMsg, p: &Peer) -> CounchError {
        CounchError::UnImpl
    }
    fn remove_peer(p: &Peer) -> CounchError {
        CounchError::UnImpl
    }

    fn register_receive_callback(model_id: &str, call_back: OnReceive) -> CounchError {
        CounchError::UnImpl
    }
}
