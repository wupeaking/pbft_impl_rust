use crate::error::CounchError;
use crate::model::MessageType;

//type OnReceive func(modelID string, msgBytes []byte, p *Peer)

pub type OnReceive = fn(model_id: String, msg: &[u8], p: &Peer);

pub trait SwitcherI {
    fn broadcast(model_id: String, msg: &BroadcastMsg) -> CounchError;
    fn broadcast_to_peer(model_id: String, msg: &BroadcastMsg, p: &Peer) -> CounchError;
    fn broadcast_except_peer(model_id: String, msg: &BroadcastMsg, p: &Peer) -> CounchError;
    fn remove_peer(p: &Peer) -> CounchError;

    fn register_receive_callback(model_id: &str, call_back: OnReceive) -> CounchError;
}

pub struct BroadcastMsg {
    pub model_id: String,
    pub msg_tye: MessageType,
    pub msg: Vec<u8>,
}

pub struct Peer {
    pub id: String,      // 定义peerid  每个peerid应该是唯一的
    pub address: String, // 地址
}
/*
type SwitcherI interface {
    // 向所有的节点广播消息
    Broadcast(modelID string, msg *BroadcastMsg) error
    // 广播到指定的peer
    BroadcastToPeer(modelID string, msg *BroadcastMsg, p *Peer) error
    // 广播 除了指定的peer
    BroadcastExceptPeer(modelID string, msg *BroadcastMsg, p *Peer) error
    // 移除某个peer
    RemovePeer(p *Peer) error
    RegisterOnReceive(modelID string, callBack OnReceive) error
    Start() error
    // 返回所有存在的peers
    Peers() ([]*Peer, error)
    // Recv() <-chan interface{}
}

type BroadcastMsg struct {
    ModelID string                 `json:"model_id"`
    MsgType model.BroadcastMsgType `json:"msg_type"`
    Msg     []byte                 `json:"msg"`
}

// OnReceive 注册接收消息回到
type OnReceive func(modelID string, msgBytes []byte, p *Peer)

type Peer struct {
    ID      string // 定义peerid  每个peerid应该是唯一的
    Address string // 地址
}

type PeerBooks struct {
    sync.RWMutex
    sets map[string]*Peer
}

func NewPeerBooks() *PeerBooks {
    return &PeerBooks{
        sets: make(map[string]*Peer),
    }
}

func (pb *PeerBooks) AddPeer(p *Peer) {
    if p == nil {
        return
    }
    pb.Lock()
    pb.sets[p.ID] = p
    pb.Unlock()
}

func (pb *PeerBooks) FindPeer(id string) *Peer {
    pb.RLock()
    defer pb.RUnlock()
    v := pb.sets[id]
    return v
}

func (pb *PeerBooks) RemovePeer(id string) {
    pb.Lock()
    defer pb.Unlock()
    delete(pb.sets, id)
}
*/
