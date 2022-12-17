
use crossbeam::channel::unbounded;
use {
    solana_sdk::packet::{Packet, PACKET_DATA_SIZE},
    solana_streamer::nonblocking::recvmmsg::recv_mmsg,
    std::net::{IpAddr, Ipv4Addr},
    tokio::net::UdpSocket,
};
use solana_client::nonblocking::{udp_client::UdpTpuConnection, tpu_connection::TpuConnection};
// use solana_client::udp_client::UdpTpuConnection;
use solana_gossip::{cluster_info::VALIDATOR_PORT_RANGE, ping_pong::{Pong, Ping}};
use solana_gossip::gossip_service;
use solana_sdk::{hash::Hash, signature::Signable, sanitize::Sanitize};
// use solana_sdk::packet::{PACKET_DATA_SIZE, Packet};
// use solana_streamer::recvmmsg::recv_mmsg;
use std::{collections::HashMap, sync::atomic::AtomicBool};
use std::net::{SocketAddr};
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
#[tokio::main]
async fn check_send_one(connection: &UdpTpuConnection, reader: &UdpSocket) {
    let packet = vec![111u8; PACKET_DATA_SIZE];
    connection.send_wire_transaction(&packet).await.unwrap();
    let mut packets = vec![Packet::default(); 32];
    let recv = recv_mmsg(reader, &mut packets[..]).await.unwrap();
    assert_eq!(1, recv);
}
type Token = [u8; 32];
// fn test_ping_pong() {
//     let mut rng = rand::thread_rng();
//     let keypair = Keypair::new();
//     let ping = Ping::<Token>::new_rand(&mut rng, &keypair).unwrap();
//     // assert!();
//     // assert!(ping.sanitize().is_ok());

//     let pong = Pong::new(&ping, &keypair).unwrap();
// }
// use solana
#[tokio::main]
async fn main() {
 
    // let socket = UdpSocket::bind("127.0.0.1:1027").unwrap();
    // let socket_address= SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1027);
    // let tpu_socket= Arc::new(socket);
    // let connection = UdpTpuConnection::new_from_addr(tpu_socket,socket_address);
    // UdpTpuConnection::
       
    let node_pubkey = pubkey::Pubkey::from_str("3u1cuv3LcNPtAgcUvZiA4k2eZwgKpj5u6Jwwzq4xfXQe").unwrap();
    let gossip_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(147,28,133,107)),8001);
    let bind_addr= IpAddr::V4(Ipv4Addr::LOCALHOST);

    let tpu_address = SocketAddr::new(bind_addr,8004);
    // println!("9");
    let node_info = Node::new_with_external_ip(&node_pubkey, &gossip_socket, VALIDATOR_PORT_RANGE, bind_addr, Some(tpu_address));

    // println!("10");
    let kp = Keypair::new();
    
    // let cluster_info = ClusterInfo::new(
    //     node_info.info,
    //     Arc::new(kp),
    //     SocketAddrSpace::Unspecified,
    // );  
    // println!("shred {:?}",cluster_info.my_shred_version());
    // let exit = Arc::new(AtomicBool::new(false));
    // let gs = gossip_service::make_gossip_node(kp, Some(&gossip_socket), &exit, Some(&tpu_address),0, false, SocketAddrSpace::Global);
// let pg = solana_gossip::ping_pong::Ping::new(token, keypair)
//    cluster_info.lookup_contact_info_by_gossip_addr(gossip_addr)
    // cluster_info.
    // let reader = UdpSocket::bind("147.28.133.107:8001").expect("bind");
    // let addr = reader.local_addr().unwrap();
    // reader.set_read_timeout(Some(Duration::new(5, 0))).unwrap();
    // reader.set_nonblocking(false).unwrap();
  
    // let sent = node_info.sockets.gossip.send(&[0, 1, 2]);
    // println!("{:?}",sent);
    // let con = node_info.sockets.gossip.
//    check_send_one(nod, reader)

    let (s, r) = unbounded();
   loop{
    let mut buf = [0; 1280];
    println!("1");
    // node_info.sockets.ancestor_hashes_requests.
    match node_info.sockets.gossip.recv_from(&mut buf) {
        Ok((k))=> {
            println!("2");
            s.send(k).unwrap();
            println!("{:?}",r.recv());
        },
        Err(e)=> println!("hello")
    };
   };
}
