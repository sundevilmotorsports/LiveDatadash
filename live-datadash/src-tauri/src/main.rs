// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::sync::Arc;
// use std::sync::Mutex;
use std::time::Duration;
use std::vec;
use serde::de::value::Error;
use serialport::COMPort;
use serialport::SerialPort;
use serialport::SerialPortInfo;
use serialport::SerialPortType;
use tauri::App;
use tauri::Manager;

use std::thread;
use std::io;
use std::io::Read;

//type SerialBuf = Arc<Mutex<Vec<Vec<i32>>>>;

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  println!("begin main");
  tauri::Builder::default()
  .setup(|app| {
    let app_handle = app.handle();
    let mut serial_buf: Vec<u8> = vec![0; 100];
    tauri::async_runtime::spawn(async move {
      let mut port : COMPort = get_port();
      println!("port: {:?}", port.name());
      port.read(serial_buf.as_mut_slice()).expect("Found no data!");
      println!("port val: {:?}", port);
       loop {
         port.read(serial_buf.as_mut_slice()).expect("Found no data!");
         app_handle.emit_all("packet", decode(serial_buf.to_vec())).unwrap();
       }
    });
    Ok(())
  })
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
  println!("end main");
}

// fn set_number(number : i32) -> i32{
  //   println!("Here");
  //   //format!("Testing {}", number)
  //   return number + 10;
  // }
  
// #[tauri::command]
// async fn get_data(app: &App){
//   println!("Inside get_data");
  
  
// }


fn get_port() -> COMPort {

  //println!("{:?}", ports);
  let curr_port : SerialPortInfo;
  let mut ports : Vec<SerialPortInfo> = vec![];
  let mut port_names : Vec<String> = vec![];

  for i in serialport::available_ports().expect("No ports found!"){
    if i.port_type != SerialPortType::Unknown{
      ports.push(i.clone());
      port_names.push(i.port_name.clone());
    }
  }
  
  if ports.len() == 0{
    panic!("No ports are available");
  }if ports.len() == 1 {
    curr_port = ports[0].clone();
    println!("len of port_vec is 1, port is {}", curr_port.port_name);
  } else {
    println!("More than 1 port is populated, choose correct port\n");
    println!("portvec {:?}\ntype {:?}", port_names, ports);
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    println!("");
    let chosen: usize = input.trim().parse().expect("Input not an integer");
    curr_port = ports[chosen].clone();
    println!("Using port {}", curr_port.port_name);
  }

  //opens serialport
  //TODO: needs to be tested on mac to see if open_native works
  
  let port = serialport::new(curr_port.port_name, 115200)
    .timeout(Duration::from_secs(5))
    .open_native()
    .expect("Failed to open port");
  return port;
}

fn decode(buf : Vec<u8>) -> Vec<i32>{
  let mut data: Vec<i32> = vec![];

  //tells program to increment index if i is a comma or space
  let mut new_index = true;
  let mut index = 0;
   
  for i in buf{
    if i as char == '\n' {
      println!("final vec {:?}", data); 
      return data;
    } else if i as char == ' '{
      continue;
    } else if i as char == ','{
      new_index = true;
      index += 1;
    }else if i.is_ascii_digit() {
      if new_index{
        //If previous character was a comma, this is a new number and needs to be pushed
        data.push((i - 48).into());
        new_index = false;
      } else {
        //Increases previous number by a factor of ten and adds current value to it
        data[index] = data[index] * 10 + <u8 as Into<i32>>::into(i - 48);
      }

    }
  }
  return data;
}