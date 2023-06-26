use std::{collections::HashMap, io, str::FromStr};

use hyper::{client::conn::SendRequest, *};
use tokio::net::TcpStream;

const MAIN_URL: &str = "http://127.0.0.1:3000/api/smarthome/v1.0";

async fn connect(url: &Uri) -> SendRequest<Body> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await.unwrap();
    let (sender, conn) = hyper::client::conn::handshake(stream).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });
    sender
}

async fn print_response(res: Response<Body>) {
    let status = res.status();
    let bytes = body::to_bytes(res.into_body()).await.unwrap();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    println!("{}\t\tBody:\n{}", status, body);
}

async fn request(path: &str, method: &str, headers: HashMap<&str, &String>) -> Response<Body> {
    let end_path = MAIN_URL.to_owned() + path;
    let url = Uri::from_str(&end_path).unwrap();
    let mut sender = connect(&url).await;

    let mut req = Request::builder().uri(url).method(method);
    for (key, value) in headers {
        req = req.header(key, value);
    }
    let req = req.body(Body::empty()).unwrap();

    sender.send_request(req).await.unwrap()
}

fn read() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read from stdin");
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

fn read_u32() -> Option<u32> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trim = buffer.trim();
    match trim.parse::<u32>() {
        Ok(i) => Some(i),
        Err(..) => None,
    }
}

enum Commands {
    Exit,                         //0
    AddRoom(String),              //1
    AddDevice(String),            //2
    RemoveRoom(String),           //3
    RemoveDevice(String, String), //4
    //UpdateDevice(String, String, HashMap<&'a str, u64>), //5
    ShowHome,                   //6
    ShowRoom(String),           //7
    ShowDevice(String, String), //8
    ShowInstruct,               //9
    Noop,
}

fn print_commands() {
    println!("\t\tcommands");
    println!("0: exit");
    println!("1: add room");
    println!("2: add device to room");
    println!("3: remove room");
    println!("4: remove device from room");
    println!("5: update device (todo)");
    println!("6: show home");
    println!("7: show room");
    println!("8: show device from room");
    println!("9: show that again");
    println!();
}

impl Commands {
    async fn exec(&self) -> Self {
        match self {
            Commands::Noop => {
                let s = read_u32();
                match s {
                    Some(0) => {
                        return Commands::Exit;
                    }
                    Some(6) => return Commands::ShowHome,
                    Some(9) => {
                        return Commands::ShowInstruct;
                    }

                    Some(x) => {
                        println!("input room name:");
                        let roomname = read();
                        match x {
                            1 => {
                                return Commands::AddRoom(roomname);
                            }
                            2 => {
                                return Commands::AddDevice(roomname);
                            }
                            3 => {
                                return Commands::RemoveRoom(roomname);
                            }
                            7 => {
                                return Commands::ShowRoom(roomname);
                            }
                            y => {
                                println!("input device name:");
                                let devicename = read();
                                match y {
                                    4 => {
                                        return Commands::RemoveDevice(roomname, devicename);
                                    }
                                    8 => {
                                        return Commands::ShowDevice(roomname, devicename);
                                    }
                                    _ => {
                                        return Commands::Noop;
                                    } /* how it posible?? */
                                }
                            }
                        }
                    }
                    // Some(5) => {
                    //     let roomname = read();
                    //     let devicename = read();

                    //     return Commands::UpdateDevice(roomname, devicename, newstat);
                    // }
                    _ => {
                        return Commands::Noop;
                    }
                }
            }
            Commands::Exit => {
                return Commands::Exit;
            }
            Commands::AddRoom(roomname) => {
                let headers: HashMap<&str, &String> = HashMap::from([("Room-Name", roomname)]);
                let res = request("/home/rooms/add", "POST", headers).await;
                print_response(res).await;
            }
            Commands::AddDevice(roomname) => {
                println!("input device name");
                let name = read();
                println!("input device type");
                let dtype = read();

                let payload = format!("\"dtype\":\"{}\",\"name\":\"{}\"", dtype, name);
                let mut device: String = "{".to_string();
                device += &payload;
                device += "}";

                let headers: HashMap<&str, &String> =
                    HashMap::from([("Room-Name", roomname), ("DeviceBuilder", &device)]);
                let res = request("/home/rooms/devices/add", "POST", headers).await;
                print_response(res).await;
            }
            Commands::RemoveRoom(roomname) => {
                let headers: HashMap<&str, &String> = HashMap::from([("Room-Name", roomname)]);
                let res = request("/home/rooms/del", "POST", headers).await;
                print_response(res).await;
            }
            Commands::RemoveDevice(roomname, devicename) => {
                let headers: HashMap<&str, &String> =
                    HashMap::from([("Room-Name", roomname), ("Device-Name", devicename)]);
                let res = request("/home/rooms/devices/del", "POST", headers).await;
                print_response(res).await;
            }
            //Commands::UpdateDevice(_, _, _) => todo!(),
            Commands::ShowHome => {
                let headers: HashMap<&str, &String> = HashMap::new();
                let res = request("/home/show", "GET", headers).await;
                print_response(res).await;
            }
            Commands::ShowRoom(roomname) => {
                let headers: HashMap<&str, &String> = HashMap::from([("Room-Name", roomname)]);
                let res = request("/home/rooms/show", "GET", headers).await;
                print_response(res).await;
            }
            Commands::ShowDevice(roomname, devicename) => {
                let headers: HashMap<&str, &String> =
                    HashMap::from([("Room-Name", roomname), ("Device-Name", devicename)]);
                let res = request("/home/rooms/devices/show", "GET", headers).await;
                print_response(res).await;
            }
            Commands::ShowInstruct => {
                print_commands();
            }
        }

        Commands::Noop
    }
}

#[tokio::main]
async fn main() {
    let mut com = Commands::Noop;
    print_commands();
    loop {
        com = com.exec().await;
        if let Commands::Exit = com {
            break;
        }
    }
}
