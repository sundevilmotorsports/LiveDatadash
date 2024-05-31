use std::cmp::max;
use std::time::Duration;
use std::vec;
use serialport::Error;
use serialport::SerialPort;
use serialport::SerialPortInfo;
use serialport::SerialPortType;

use std::io;
use std::io::Read;

const BAUD : u32 = 9600;
const TIMEOUT : u64 = 1000;
const PACKET_COUNT : usize = 3;

fn main() {
  let connection = sqlite::open("../server/ldd.db").unwrap();
  let init = "
    delete from imu;
    delete from wheel;
    delete from datalog;
    delete from ack;
  ";
  connection.execute(init).unwrap();

  let mut prev : Vec<Vec<f32>> = vec![vec![0.0;5]; PACKET_COUNT];
  let mut serial_buf: Vec<u8> = vec![0; 1000];
  let port_result : Result<serialport::COMPort, Error> = get_port();
  let mut port : serialport::COMPort;

  if port_result.is_ok(){
    port = port_result.unwrap();
  } else {
    panic!("could not connect to port: {:?}", port_result.err());
  }
  println!("port val: {:?}", port);
  println!("entering loop");
  while port.bytes_to_read().unwrap() == 0{
    println!("waiting for data: {:?}", port.bytes_to_read().unwrap());
  };
  loop {
    if port.bytes_to_read().unwrap() > 21{
      println!("reading");
      let read : Result<usize, io::Error> = port.read(&mut serial_buf);
      println!("past read");
      if read.is_ok() {
        let temp: Vec<Vec<f32>> = decode(serial_buf.clone());
        println!("past decode");
        if temp[0][1] != prev[0][1]{
          prev[0] = temp[0].clone();
          let imu : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO imu VALUES (1, {}, {}, {}, {}, {}, {}, {});", temp[0][1], temp[0][2], temp[0][3], temp[0][4], temp[0][5], temp[0][6], temp[0][7]));
          print!("imu packet sent: {}", imu.is_ok());
        } 
        if temp[1][1] != prev[1][1]{
          prev[1] = temp[1].clone();
          let wheel : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO wheel VALUES (2, {}, {}, {}, {}, {}, {}, {},{}, {}, {}, {}, {}, {});", temp[1][1], temp[1][2], temp[1][3], temp[1][4], temp[1][5], temp[1][6], temp[1][7], temp[1][8], temp[1][9], temp[1][10], temp[1][11], temp[1][12], temp[1][13]));
          print!("imu packet sent: {}", wheel.is_ok());
        }
        if temp[2][1] != prev[2][1]{
          prev[2] = temp[2].clone();
          let datalog : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO datalog VALUES (3, {}, {}, {}, {}, {}, {}, {},{}, {}, {});", temp[2][1], temp[2][2], temp[2][3], temp[2][4], temp[2][5], temp[2][6], temp[2][7], temp[2][8], temp[2][9], temp[2][10]));
          print!("imu packet sent: {}", datalog.is_ok());
        }
          println!("buffer value: {serial_buf:?}");
      } else {
        println!("error: {:?}", read.err())
      }
    }
  }
}

fn get_port() -> Result<serialport::COMPort, Error>{
  
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
    //needs flow control or can't read COM port, idk why :)
  let port : Result<serialport::COMPort, Error>  = serialport::new(curr_port.port_name, BAUD)
  .timeout(Duration::from_millis(TIMEOUT))
  .flow_control(serialport::FlowControl::Hardware)
  .open_native();
  return port;
}

  //TODO: Implement
// fn recconnect(portname : String) -> Box<Result<Box<dyn SerialPort>, Error>> {
//   Box::new(serialport::new(portname, BAUD).timeout(Duration::from_millis(TIMEOUT)).open())
// }

fn decode(buf : Vec<u8>) -> Vec<Vec<f32>>{
  let mut final_vec : Vec<Vec<f32>> = vec![vec![]; PACKET_COUNT];
  let mut vec_index: usize = 0;
  let mut index: usize = 0;
  let mut decimal: bool = false;
  let mut decimal_counter = 0;
  
  for i in buf{
    if i as char == '\n'{
      index = 0;
      decimal = false;
      decimal_counter = 0;

      vec_index += 1;
      if vec_index == PACKET_COUNT{
        break;
      }
    } else if i as char == '.' {
        decimal = true;
    }  else if i as char == ',' {
        final_vec[vec_index][index] /= max(1, decimal_counter * 10) as f32;
        final_vec[vec_index].push(0.0);
        index += 1;
    }else if i.is_ascii_digit() {
      if decimal{
        decimal_counter += 1;
      }
      if final_vec[vec_index].len() == 0{
        final_vec[vec_index].push((i-48).into());
      } else {
        final_vec[vec_index][index] = final_vec[vec_index][index] * 10.0 + (i as f32 - 48.0);
      }
    }
  }
  return final_vec;
}