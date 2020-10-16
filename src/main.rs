use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use serde_json::{Result, Value};
use serde::Serialize;

/*
OP Codes:

0 - Request Auth
1 - Send auth

2 - request ack/heartbeat
3 - heartbeat

4 - tell client of server error
5 - tell client of client error

6 - send query
7 - query res

*/

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "op", content = "data", rename_all = "camelCase")]
enum OpData {

    #[serde(rename = "0")]
    AuthReq {
        message: String
    },

    #[serde(rename = "1")]
    AuthRes {
        database: String,
        key: String
    },

    #[serde(rename = "2")]
    ServerError {
        error: String
    },

    #[serde(rename = "3")]
    ClientError {
        error: String
    },

    #[serde(rename = "4")]
    ReqQuery {

    },

    #[serde(rename = "5")]
    ResQuery {

    },

    #[serde(rename = "6")]
    AuthValidate {
        success: bool
    },

}

fn client_con_err() {
    println!("[WAVE] client connection failed!")
}

fn client_closed_err() {
    println!("[WAVE] client forcibly closed connection!")
}

fn handle_client(mut stream: TcpStream) {
    println!("[WAVE] new client connected ['ip': '{}', 'name': 'unknown'] ", stream.peer_addr().unwrap().ip());

    let op_0_data = OpData::AuthReq { message: "please authenticate".parse().unwrap() };
    let _ = stream.write(serde_json::to_string(&op_0_data).unwrap().as_ref());

    loop {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Err(e) => {
                client_closed_err();
                break;
            }
            _ => {}
        }

        let parsed_buffer = String::from_utf8_lossy(&buffer[..]);
        println!("BUFFER: {}", parsed_buffer);

        let string_data = r#""# + parsed_buffer;

        if !parsed_buffer.is_empty() {
            match  serde_json::from_str::<OpData>(string_data) {
                  Err(_) => {
                        let op_5_data = OpData::ClientError { error: "Failed to extract valid op-code".parse().unwrap() };
                        let _ =  stream.write(serde_json::to_string(&op_5_data).unwrap().as_ref());
                  },
                  Ok(OpData::AuthRes { database, key }) => {
                        let op_6_data = OpData::AuthValidate { success: true };
                        println!("[WAVE] recieved op 1 to login: successful!");
                        let _ = stream.write(serde_json::to_string(&op_6_data).unwrap().as_ref());
                  }
                  _ => {
                      println!("[WAVE] unknown op code!")
                  }
            };
        }
    }
}

fn main() {
    println!("[WAVE] Online and running at 127.0.0.1:2222");
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { std::thread::spawn(move || handle_client(stream)); },
            Err(_e) => client_con_err(),
        }
    }
}
