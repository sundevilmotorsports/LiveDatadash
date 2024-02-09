// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use core::num;
use std::time::Duration;

fn main() {
  number_test("10");
  serial_test();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .invoke_handler(tauri::generate_handler![number_test])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn number_test(number: &str) -> String{
  format!("Testing {}", number)
}



fn serial_test(){
  let ports = serialport::available_ports().expect("No ports found!");
  let mut temp_port = String::from("COM6");
  for p in ports {
      println!("{}", p.port_name);
      temp_port = p.port_name;
  }
  println!("{}",temp_port);
  //port.write_data_terminal_ready(true);
  let mut port = serialport::new(temp_port, 115200)
    .timeout(Duration::from_secs(10))
    .open()
    .expect("Failed to open port");

  let mut serial_buf: Vec<u8> = vec![0; 32];
  port.read(serial_buf.as_mut_slice()).expect("Found no data!");
  println!("{:?}", serial_buf);
}