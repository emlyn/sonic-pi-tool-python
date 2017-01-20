use std::net::UdpSocket;
use std::net;
use rosc::{OscPacket, OscMessage, OscType};
use rosc::encoder;

type OscMsg = Vec<u8>;


/// Takes a string of Sonic Pi source code and sends it to the Sonic Pi server.
///
pub fn run_code(source: String) {
    let client_name = OscType::String("SONIC_PI_TOOL".to_string());
    let osc_source = OscType::String(source);

    let msg = &OscPacket::Message(OscMessage {
        addr: "/run-code".to_string(),
        args: Some(vec![client_name, osc_source]),
    });
    let msg_buf = encoder::encode(msg).unwrap();
    send(msg_buf);
}


/// Instuct the Sonic Pi server to stop playing.
///
pub fn stop_all_jobs() {
    let client_name = OscType::String("SONIC_PI_TOOL".to_string());

    let msg = &OscPacket::Message(OscMessage {
        addr: "/stop-all-jobs".to_string(),
        args: Some(vec![client_name]),
    });
    let msg_buf = encoder::encode(msg).unwrap();
    send(msg_buf);
}


/// Send an OSC message to the Sonic Pi server, which is presumed to be
/// listening on UDP port 4557.
///
/// We don't expect to recieve anything, so we bind to 0.0.0.0:0, which prompts
/// the OS to assign us an arbitrary unused port.
///
fn send(msg: OscMsg) {
    let localhost = net::Ipv4Addr::new(127, 0, 0, 1);
    let s_pi_addr = net::SocketAddrV4::new(localhost, 4557);

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(&msg, s_pi_addr).unwrap();
}
