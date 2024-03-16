// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::vec;
use serialport::COMPort;
use serialport::SerialPort;
use serialport::SerialPortInfo;
use tauri::State;

use std::thread;
use std::io;
use std::io::Read;

type SerialBuf = Arc<Mutex<Vec<Vec<i32>>>>;



fn main() {
  //setGreeting("10".to_string());
  let useable_data = Arc::new(Mutex::new(vec![vec![]]));
  let useable_data_1 = useable_data.clone();
  let mut i = 0;
  let mut runner = thread::spawn(move || {
    let mut port = get_port();
    let mut serial_buf: Vec<u8> = vec![0; 100];
    let mut ready : Result<(), serialport::Error> = port.write_data_terminal_ready(true);

    while ready.is_ok(){
      i += 1;
      println!("thread running {}", i);
      //Uses ASCII Chars, 48-57 = 0-9, 44 = ',', EOL = 13 = carriage return (\n)
      //convert to char with "_ as char"

      //Reads until buffer is full, if simulating pading at end of string after the \n
      port.read(serial_buf.as_mut_slice()).expect("Found no data!");
      let useable_data_inner = Arc::clone(&useable_data_1);
      let mut guard = useable_data_inner.lock().unwrap();
      guard.push(decode(serial_buf.to_vec()));
      drop(guard);
      ready = port.write_data_terminal_ready(false);
    }
  });

  tauri::Builder::default()
    .manage(useable_data)
    .invoke_handler(tauri::generate_handler![get_data, set_number])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn set_number(number : i32) -> i32{
  println!("Here");
  //format!("Testing {}", number)
  return number + 10;
}



#[tauri::command]
fn get_data(useable_data: State<'_, SerialBuf>) -> String {
    println!("Inside get_data");
    join().
    let guard = useable_data.lock().unwrap();
    let curr_used = guard.clone();
    println!("val of curr : {:?}", guard);
    drop(guard);
    format!("testing first num: {:?}", curr_used)
}


fn get_port() -> COMPort {
  let ports = serialport::available_ports().expect("No ports found!");

  println!("{:?}", ports);

  let curr_port : SerialPortInfo;
  
  if ports.len() == 0{
    panic!("No ports are available");
  }if ports.len() == 1 {
    curr_port = ports[0].clone();
    println!("len of port_vec is 1, port is {}", curr_port.port_name);
    
  } else {
    //TODO: Prompt the user for what port to use 
    //implement serialport::SerialPortInfo.port_type or serialport::USBPortInfo (finding different info about ports to make it user friendly)
    println!("else {:?}", ports);
    println!("portvec {}", ports[0].port_name);
    println!("More than 1 port is populated, choose correct port");
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
    .timeout(Duration::from_secs(100))
    .open_native()
    .expect("Failed to open port");
  return port;
}

fn decode(buf : Vec<u8>) -> Vec<i32>{
  let mut data: Vec<i32> = vec![];

  //tells program to increment index if i is a comma
  //I hate this but couldn't think of another way
  let mut new_index = true;
  let mut index = 0;

  
  // Real code
   
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

  //for debugging without arduino
  /*for i in buf{
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
  }*/
  return data;
}