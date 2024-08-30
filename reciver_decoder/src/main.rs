use std::time::Duration;
use std::vec;
use serialport::SerialPortInfo;
use serialport::SerialPortType;
use rand::Rng;
use std::thread;

use std::io;
use std::io::Read;

const BAUD : u32 = 9600;
const TIMEOUT : u64 = 2500;
const PACKET_COUNT : usize = 3;
const TESTING : bool = false;

fn main() {
  let connection = sqlite::open("../../server/ldd.db").unwrap();
  let init = "
        DROP TABLE IF EXISTS imu;
        DROP TABLE IF EXISTS wheel;
        DROP TABLE IF EXISTS datalog;
        DROP TABLE IF EXISTS ack;

        CREATE TABLE imu(
            id INTEGER NOT NULL, 
            timestamp INTEGER NOT NULL PRIMARY KEY,
            x_acceleration INTEGER NOT NULL, 
            y_acceleration INTEGER NOT NULL, 
            z_acceleration INTEGER NOT NULL, 
            x_gyro INTEGER NOT NULL, 
            y_gyro INTEGER NOT NULL, 
            z_gyro INTEGER NOT NULL
        );

        CREATE TABLE wheel(
            id INTEGER NOT NULL, 
            timestamp INTEGER NOT NULL PRIMARY KEY, 
            fl_wheel_speed INTEGER NOT NULL, 
            fl_brake_temp REAL NOT NULL, 
            fl_ambiant_temp REAL NOT NULL,
            fr_wheel_speed INTEGER NOT NULL, 
            fr_brake_temp REAL NOT NULL, 
            fr_ambiant_temp REAL NOT NULL,
            rl_wheel_speed INTEGER NOT NULL, 
            rl_brake_temp REAL NOT NULL, 
            rl_ambiant_temp REAL NOT NULL,
            rr_wheel_speed INTEGER NOT NULL, 
            rr_brake_temp REAL NOT NULL, 
            rr_ambiant_temp REAL NOT NULL
        );

        CREATE TABLE datalog(
            id INTEGER NOT NULL, 
            timestamp INTEGER NOT NULL PRIMARY KEY,
            drs INTEGER NOT NULL, 
            steering_angle INTEGER NOT NULL, 
            throttle_input REAL NOT NULL,
            front_brake_pressure REAL NOT NULL, 
            rear_brake_pressure REAL NOT NULL,
            gps_lattitude REAL NOT NULL, 
            gps_longitude REAL NOT NULL, 
            battery_voltage REAL NOT NULL, 
            daq_current_draw REAL NOT NULL
        );

        CREATE TABLE ack(
            id INTEGER NOT NULL, 
            timestamp INTEGER NOT NULL PRIMARY KEY
        );
  ";
  let initial_connection_result : Result<(), sqlite::Error> = connection.execute(init);
  println!("Does the initial connection work??: {}", initial_connection_result.is_ok());

  let mut prev : Vec<Vec<f32>> = vec![vec![0.0;5]; PACKET_COUNT];
  let mut serial_buf: Vec<u8> = vec![0; 1000];
  let port_result: Result<serialport::COMPort, &str> = get_port();
  let mut port = None;

  match port_result {
    Ok(p) => {
        port = Some(p);
        println!("port val: {:?}", port);
        
    }
    Err(e) => {
        println!("Error opening port: {:?}", e);
        println!("No ports available. Generating fake data.");
    }
  }

  loop {
    let payload: Vec<Vec<f32>>;
    if let Some(ref mut port) = port {
      if !TESTING {
        //println!("reading");
        let read : Result<usize, io::Error> = port.read(&mut serial_buf);
        //println!("past read");
        if read.is_ok() {
          payload = decode(serial_buf.clone());
          //println!("past decode");
          //println!("payload value: {payload:?}");
        } else {
          println!("error: {:?}", read.err());
          continue;
        }
      } else {
        // In the event that the read is longer than expected i.e it is wrong and invalid
        //println!("Current read from port is not valid, too long");
        continue;
      }
    } else {
      payload = generate_fake_data();
      thread::sleep(Duration::from_millis(4000));
    };
    prev = write_db(payload, prev, &connection);
  }
}

fn write_db(payload: Vec<Vec<f32>>, mut prev : Vec<Vec<f32>>, connection : &sqlite::Connection) -> Vec<Vec<f32>>{
  if payload[0][1] != prev[0][1]{
    prev[0] = payload[0].clone();
    let imu : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO imu VALUES (1, {}, {}, {}, {}, {}, {}, {});", payload[0][1], payload[0][2], payload[0][3], payload[0][4], payload[0][5], payload[0][6], payload[0][7]));
    if imu.is_ok(){
      print!("imu packet recived,\t");
    } else {
      print!("imu packet err: {:?},\t", imu.err().unwrap().to_string());
    }
  } 
  if payload[1][1] != prev[1][1]{
    prev[1] = payload[1].clone();
    let wheel : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO wheel VALUES (2, {}, {}, {}, {}, {}, {}, {},{}, {}, {}, {}, {}, {});", payload[1][1], payload[1][2], payload[1][3], payload[1][4], payload[1][5], payload[1][6], payload[1][7], payload[1][8], payload[1][9], payload[1][10], payload[1][11], payload[1][12], payload[1][13]));
    if wheel.is_ok() {
      print!("wheel packet recived,\t");
    } else {
      print!("wheel packet err: {:?},\t", wheel.err().unwrap().to_string());
    }
  }
  if payload[2][1] != prev[2][1]{
    prev[2] = payload[2].clone();
    let datalog : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO datalog VALUES (3, {}, {}, {}, {}, {}, {}, {},{}, {}, {});", payload[2][1], payload[2][2], payload[2][3], payload[2][4], payload[2][5], payload[2][6], payload[2][7], payload[2][8], payload[2][9], payload[2][10]));
    if datalog.is_ok(){
      println!("datalog packet recived");
    } else {
      println!("datalog err: {:?} ", datalog.err().unwrap().to_string());
    }
  }
  return prev;
}

fn get_port() -> Result<serialport::COMPort, &'static str> {
  let curr_port : SerialPortInfo;
  let mut ports : Vec<SerialPortInfo> = vec![];
  let mut port_names : Vec<String> = vec![];

  for i in serialport::available_ports().expect("No ports found!") {
    if i.port_type != SerialPortType::Unknown{
      ports.push(i.clone());
      port_names.push(i.port_name.clone());
    }
  }
  
  if ports.len() == 0 {
    return Err("No ports are available");
  } if ports.len() == 1 {
    curr_port = ports[0].clone();
    println!("len of port_vec is 1, port is {}", curr_port.port_name);
    let port = serialport::new(curr_port.port_name, BAUD)
      .timeout(Duration::from_millis(TIMEOUT))
      .flow_control(serialport::FlowControl::Hardware)
      .open_native();
    return Ok(port.unwrap())
  } else {
    println!("More than 1 port is populated, choose correct port\n");
    println!("portvec {:?}\ntype {:?}", port_names, ports);
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    println!("");
    let chosen: usize = input.trim().parse().expect("Input not an integer");
    curr_port = ports[chosen].clone();
    println!("Using port {}", curr_port.port_name);

    let port = serialport::new(curr_port.port_name, BAUD)
      .timeout(Duration::from_millis(TIMEOUT))
      .flow_control(serialport::FlowControl::Hardware)
      .open_native();
  return Ok(port.unwrap())
  }
}

fn generate_fake_data() -> Vec<Vec<f32>> {
  let mut rng = rand::thread_rng();
  let mut fake_data = Vec::new();
  for _ in 0..PACKET_COUNT {
      let mut packet = Vec::new();
      print!("New packet: [");
      for _ in 0..14 {
          let value = (rng.gen::<f32>() * 100.0).round();
          packet.push(value);
          print!("{:.2} ", value);
      }
      fake_data.push(packet);
      println!("] ");
  }
  return fake_data;
}

fn decode(buf : Vec<u8>) -> Vec<Vec<f32>>{
  let mut final_vec : Vec<Vec<f32>> = vec![vec![]; PACKET_COUNT];
  let mut vec_index: usize = 0;
  let mut index: usize = 0;
  let mut decimal: bool = false;
  let mut negitive: bool = false;
  let mut decimal_counter: u32 = 0;
  //println!("{:?}", buf);
  for i in buf{
    if i as char == '\n'{
      index = 0;
      decimal = false;
      negitive = false;
      decimal_counter = 0;

      vec_index += 1;
      if vec_index == PACKET_COUNT{
        break;
      }
    } else if i as char == '-'{
        negitive = true;
    } else if i as char == '.' {
        decimal = true;
    }  else if i as char == ',' {
        final_vec[vec_index][index] /= (10 as i32).pow(decimal_counter) as f32;
        if negitive{
          final_vec[vec_index][index] *= -1.0;
        }
        decimal = false;
        decimal_counter = 0;
        negitive = false;
        final_vec[vec_index].push(0.0);
        index += 1;
    } else if i.is_ascii_digit() {
      if decimal{
        decimal_counter += 1;
      }
      if final_vec[vec_index].len() == 0{
        final_vec[vec_index].push((i-48).into());
      } else {
        final_vec[vec_index][index] = final_vec[vec_index][index] * 10.0 + (i - 48) as f32;
      }
    }
  }
  return final_vec;
}