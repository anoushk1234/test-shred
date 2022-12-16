
// use solana_client::udp_client::UdpTpuConnection;
use solana_gossip::cluster_info::VALIDATOR_PORT_RANGE;
use solana_gossip::crds::Cursor;
use std::net::{UdpSocket,SocketAddr,IpAddr, Ipv4Addr,};
use std::str::FromStr;
use std::sync::Arc;
use solana_sdk::{
    pubkey,
    signature::Keypair,
    timing::{timestamp, AtomicInterval},
};
use solana_streamer::socket::SocketAddrSpace;
use  solana_gossip::{
    cluster_info::{ClusterInfo, Node}
};
// use solana
fn main() {
 
    // let socket = UdpSocket::bind("127.0.0.1:1027").unwrap();
    // let socket_address= SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1027);
    // let tpu_socket= Arc::new(socket);
    // let connection = UdpTpuConnection::new_from_addr(tpu_socket,socket_address);
    // UdpTpuConnection::
    let node_pubkey = pubkey::Pubkey::from_str("3u1cuv3LcNPtAgcUvZiA4k2eZwgKpj5u6Jwwzq4xfXQe").unwrap();
    let gossip_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),17790);
  
    let bind_addr= IpAddr::V4(Ipv4Addr::new(147,28,133,107));
    println!("");
    let tpu_address = SocketAddr::new(bind_addr,8004);
    println!("9");
    let node_info = Node::new_with_external_ip(&node_pubkey, &gossip_socket, VALIDATOR_PORT_RANGE, bind_addr, Some(tpu_address));
    println!("10");
    let cluster_info = ClusterInfo::new(
        node_info.info,
        Arc::new(Keypair::new()),
        SocketAddrSpace::Unspecified,
    );  
    
    
   let peers= cluster_info.all_peers();
    println!("{:?}", peers);
}
