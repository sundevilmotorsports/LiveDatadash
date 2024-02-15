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
      //not implemented, looks to determine how the port is connected and give user more info on which one to choose
      //Need a physical board to test
      /*if p.port_type == USBPort {
        continue;
      }*/
      println!("{}", p.port_name);
      port_vec.push(p.port_name);
  }

  let mut temp_port = String::new();
  
  if port_vec.len() == 1 {
    temp_port = port_vec[0].to_string();
    println!("len of port_vec is 1, port is {}", temp_port);
    
  } else {
    //TODO: Prompt the user for what port to use 
    //implement serialport::SerialPortInfo.port_type or serialport::USBPortInfo (finding different info about ports to make it user friendly)
    println!("{:?}", port_vec);
    println!("{}", port_vec[0]);
    println!("More than 1 port is populated, choose correct port");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    println!("");
    let chosen: usize = input.trim().parse().expect("Input not an integer");
    temp_port = port_vec[chosen].to_string();
    println!("Using port {}", temp_port);
  }
  
  //opens serialport
  //TODO: needs to be tested on mac to see if open_native works
  let mut port = serialport::new(temp_port, 115200)
    .timeout(Duration::from_secs(100))
    .open_native()
    .expect("Failed to open port");

  let mut ready = port.write_data_terminal_ready(true);

  //haven't calculated max length but in testing 50 chars was reached fairly quickly
  let mut serial_buf: Vec<u8> = vec![0; 100];

  //Unsure if needed, implemented for debugging
  let mut cleared = port.clear(serialport::ClearBuffer::All);

  let mut usable_data : Vec<Vec<i32>> = vec![];

  
  //Threads runs as long as it can read data and another part of the program is running
  thread::spawn(move || {
    while ready.is_ok() {

      //Uses ASCII Chars, 48-57 = 0-9, 44 = ',', EOL = 13 = carriage return (\n)
      //convert to char with "_ as char"

      //Reads until buffer is full, if simulating padd at end of string after the \n
      port.read(serial_buf.as_mut_slice()).expect("Found no data!");

      usable_data.push(decode(serial_buf.to_vec()));
      ready = port.write_data_terminal_ready(true);
      
      cleared = port.clear(serialport::ClearBuffer::All);
    }
  });
}

fn decode(buf : Vec<u8>) -> Vec<i32>{
  let mut data: Vec<i32> = vec![];

  //tells program to increment index if i is a comma
  //I hate this but couldn't think of another way
  let mut new_index = true;
  let mut index = 0;

  //println!("Innitaial buffer: {:?}", buf);

  for i in buf{
    //extra or is for testing without arduino, remove in final
    if i as char == '\n' /*|| i as char == '\\'*/ {
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