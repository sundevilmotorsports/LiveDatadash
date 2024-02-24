// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use std::vec;
use serialport::COMPort;
use serialport::SerialPort;
use serialport::SerialPortInfo;

use std::thread;
use std::io;
use std::io::Read;


fn main() {
  number_test("10"); 
  let mut useable_data : &Vec<Vec<i32>> = &vec![];
  threader(useable_data);
  thread::spawn(|| {
    println!("{:?}", useable_data);
    let mut port = get_port();
    let mut serial_buf: Vec<u8> = vec![0; 100];
    println!("In thread");
    let mut ready : Result<(), serialport::Error> = port.write_data_terminal_ready(true);
    let mut temp : Vec<i32> = vec![];
    while ready.is_ok(){
      
      //Uses ASCII Chars, 48-57 = 0-9, 44 = ',', EOL = 13 = carriage return (\n)
      //convert to char with "_ as char"

      //Reads until buffer is full, if simulating padd at end of string after the \n
      port.read(serial_buf.as_mut_slice()).expect("Found no data!");
      useable_data.push(decode(serial_buf.to_vec()));
      ready = port.write_data_terminal_ready(false);
      //println!("can we read this {:?}", temp);
      //return temp;
      //cleared = port.clear(serialport::ClearBuffer::All);
    }
  });
  /*tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![number_test])
    //.invoke_handler(tauri::generate_handler![serial_testing])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");*/
}

#[tauri::command]
fn number_test(number: &str) -> String{
  format!("Testing {}", number)
}

/*#[tarui::command]
fn serial_testing(vector: &Vec<i32>) -> String{
  format!("Test vec {:?}", vector);
}*/

fn get_port() -> COMPort {
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

  let mut curr_port = String::new();
  
  if port_vec.len() == 0{
    panic!("No ports are available");
  }if port_vec.len() == 1 {
    curr_port = port_vec[0].to_string();
    println!("len of port_vec is 1, port is {}", curr_port);
    
  } else {
    //TODO: Prompt the user for what port to use 
    //implement serialport::SerialPortInfo.port_type or serialport::USBPortInfo (finding different info about ports to make it user friendly)
    println!("else {:?}", port_vec);
    println!("portvec {}", port_vec[0]);
    println!("More than 1 port is populated, choose correct port");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    println!("");
    let chosen: usize = input.trim().parse().expect("Input not an integer");
    curr_port = port_vec[chosen].to_string();
    println!("Using port {}", curr_port);
  }

  //opens serialport
  //TODO: needs to be tested on mac to see if open_native works
  
  let mut port = serialport::new(curr_port, 115200)
    .timeout(Duration::from_secs(100))
    .open_native()
    .expect("Failed to open port");
  return port;
}

fn threader<'a>(address : &Vec<Vec<i32>>){

}

fn decode(buf : Vec<u8>) -> Vec<i32>{
  let mut data: Vec<i32> = vec![];

  //tells program to increment index if i is a comma
  //I hate this but couldn't think of another way
  let mut new_index = true;
  let mut index = 0;

  
  // Real code
   
  /*for i in buf{
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
  }*/

  //for debugging without arduino
  for i in buf{
    if i as char == '\\' {
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