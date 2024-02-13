// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use core::num;
use std::time::Duration;
use serialport::SerialPort;

use std::thread;
use std::io;
use std::io::Read;


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
  let mut port_vec = vec![];
  
  //Itterating though the available ports, printing then pushing them to port vector
  //  TODO: Could be possible to skip step and set port_vec equal to new serialport function
  for p in ports {
      println!("{}", p.port_name);
      port_vec.push(p.port_name);
  }

  //TODO: have temp_port be a string that isn't pre-defined
  let mut temp_port = String::new();
  
  if port_vec.len() == 1 {
    temp_port = port_vec[0].to_string();
    println!("len of port_vec is 1, port is {}", temp_port);
    
  } else {
    //TODO: Prompt the user for what port to use 
    //implement serialport::SerialPortInfo.port_type or serialport::USBPortInfo (finding different info about ports to make it user friendly)
    
    println!("More than 1 port is being occupied, choose correct port");
    print!("{:?}", port_vec);
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    println!("");
    let chosen: usize = input.trim().parse().expect("Input not an integer");
    println!("Testing {}", chosen);
    temp_port = port_vec[chosen].to_string();
    //input.as_bytes().to_vec()[0]
    println!("Using port {}", temp_port);
  }
  
  //opens serialport, needs to be tested on mac to see if open_native works
  //println!("Using port {}", temp_port);
  let mut port = serialport::new(temp_port, 115200)
    .timeout(Duration::from_secs(1))
    .open_native()
    .expect("Failed to open port");

  //defining booleans outside thread to save memmory 
  //possible it could not matter since it's being moved inside the thread

  let mut ready = port.write_data_terminal_ready(true);
  let mut serial_buf: Vec<u8> = vec![0; 50];
  //Unsure if needed, implemented for debugging
  //let mut cleared = port.clear(serialport::ClearBuffer::Input);
  let mut usable_data : Vec<Vec<i32>> = vec![];
  //Runs as long as it can read data and another part of the program is running
  thread::spawn(move || {
    while ready.is_ok() {
      port.read(serial_buf.as_mut_slice()).expect("Found no data!");
      
      //IMPORTANT: USES ASCII CHARS, 48-57 = 0-9, 44 = ',', EOL = 13 = carriage return (\n)
      //convert to char with _ as char
      //print!("index: {},", serial_buf[0]-48);
      usable_data.push(decode(serial_buf.to_vec()));
      
      //cleared = port.clear(serialport::ClearBuffer::Input);
      ready = port.write_data_terminal_ready(true);
    }
  });
}

fn decode(buf : Vec<u8>) -> Vec<i32>{
  //returning vector
  let mut data: Vec<i32> = vec![];

  //tells program to increment index if i is a comma
  //I hate this but couldn't think of another way
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
        data.push((i - 48).into());
        new_index = false;
      } else {
        data[index] = data[index] * 10 + <u8 as Into<i32>>::into(i - 48);
      }

    }
    //print!("{}", i as char);
  }
  return data;
}