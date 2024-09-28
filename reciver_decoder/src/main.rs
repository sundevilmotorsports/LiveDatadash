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
const TESTING : bool = false;

fn main() {
  let mode:i32;
  loop{
    println!("What mode are the radios in?");
    println!("0 = general, 1 = suspension, 2 = damper, 3 = driver, 4 = slip/slide");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let choice:Result<i32, std::num::ParseIntError> = input.trim().parse();
    if choice.is_ok(){
      mode = choice.unwrap();
      break;
    } else {
      println!("Invalid number");
    }
  }
  let connection = sqlite::open("../../server/ldd.db").unwrap();
  let init = "
        DROP TABLE IF EXISTS general;
        DROP TABLE IF EXISTS suspension;
        DROP TABLE IF EXISTS damper;
        DROP TABLE IF EXISTS drive;
        DROP TABLE IF EXISTS slide;

        CREATE TABLE general(
            timestamp INTEGER NOT NULL PRIMARY KEY,
            x_acceleration INTEGER NOT NULL, 
            y_acceleration INTEGER NOT NULL, 
            z_acceleration INTEGER NOT NULL, 
            x_gyro INTEGER NOT NULL, 
            y_gyro INTEGER NOT NULL, 
            z_gyro INTEGER NOT NULL,
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
            rr_ambiant_temp REAL NOT NULL,
            diff_speed INTEGER NOT NULL,
            drs INTEGER NOT NULL, 
            steering_angle INTEGER NOT NULL, 
            throttle_input REAL NOT NULL,
            front_brake_pressure REAL NOT NULL, 
            rear_brake_pressure REAL NOT NULL,
            gps_lattitude REAL NOT NULL, 
            gps_longitude REAL NOT NULL, 
            battery_voltage REAL NOT NULL, 
            daq_current_draw REAL NOT NULL,
            fr_shock_pot REAL NOT NULL,
            fl_shock_pot REAL NOT NULL,
            rr_shock_pot REAL NOT NULL,
            rl_shock_pot REAL NOT NULL
        );

        CREATE TABLE suspension(
            timestamp INTEGER NOT NULL PRIMARY KEY,
            x_acceleration INTEGER NOT NULL, 
            y_acceleration INTEGER NOT NULL, 
            z_acceleration INTEGER NOT NULL, 
            x_gyro INTEGER NOT NULL, 
            y_gyro INTEGER NOT NULL, 
            z_gyro INTEGER NOT NULL,
            fl_wheel_speed INTEGER NOT NULL,
            fr_wheel_speed INTEGER NOT NULL,
            rl_wheel_speed INTEGER NOT NULL, 
            rr_wheel_speed INTEGER NOT NULL,
            diff_speed INTEGER NOT NULL,
            drs INTEGER NOT NULL, 
            steering_angle INTEGER NOT NULL, 
            throttle_input REAL NOT NULL,
            front_brake_pressure REAL NOT NULL, 
            rear_brake_pressure REAL NOT NULL,
            gps_lattitude REAL NOT NULL, 
            gps_longitude REAL NOT NULL, 
            fr_shock_pot REAL NOT NULL,
            fl_shock_pot REAL NOT NULL,
            rr_shock_pot REAL NOT NULL,
            rl_shock_pot REAL NOT NULL
        );

        CREATE TABLE damper( 
            timestamp INTEGER NOT NULL PRIMARY KEY,
            x_acceleration INTEGER NOT NULL, 
            y_acceleration INTEGER NOT NULL, 
            z_acceleration INTEGER NOT NULL, 
            fl_wheel_speed INTEGER NOT NULL,       
            fr_shock_pot REAL NOT NULL,
            fl_shock_pot REAL NOT NULL,
            rr_shock_pot REAL NOT NULL,
            rl_shock_pot REAL NOT NULL
        );

        CREATE TABLE drive(
            timestamp INTEGER NOT NULL PRIMARY KEY,
            x_acceleration INTEGER NOT NULL, 
            y_acceleration INTEGER NOT NULL, 
            fl_wheel_speed INTEGER NOT NULL,
            rl_wheel_speed INTEGER NOT NULL,
            rr_wheel_speed INTEGER NOT NULL,
            diff_speed INTEGER NOT NULL,
            drs INTEGER NOT NULL, 
            steering_angle INTEGER NOT NULL, 
            throttle_input REAL NOT NULL,
            front_brake_pressure REAL NOT NULL, 
            rear_brake_pressure REAL NOT NULL, 
            fr_shock_pot REAL NOT NULL,
            fl_shock_pot REAL NOT NULL,
            rr_shock_pot REAL NOT NULL,
            rl_shock_pot REAL NOT NULL
        );

        CREATE TABLE slide( 
            timestamp INTEGER NOT NULL PRIMARY KEY,
            x_acceleration INTEGER NOT NULL, 
            y_acceleration INTEGER NOT NULL, 
            fl_wheel_speed INTEGER NOT NULL,
            fr_wheel_speed INTEGER NOT NULL,
            rl_wheel_speed INTEGER NOT NULL,
            rr_wheel_speed INTEGER NOT NULL,
            diff_speed INTEGER NOT NULL,
            steering_angle INTEGER NOT NULL, 
            throttle_input REAL NOT NULL,
            front_brake_pressure REAL NOT NULL, 
            rear_brake_pressure REAL NOT NULL
        );
  ";
  let initial_connection_result : Result<(), sqlite::Error> = connection.execute(init);
  println!("Does the initial connection work??: {}", initial_connection_result.is_ok());
  if initial_connection_result.is_err() {
    println!("{}", initial_connection_result.err().unwrap());
  }

  let mut prev : Vec<Vec<f32>> = vec![];
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
          payload = decode(mode, serial_buf.clone());
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
      payload = generate_fake_data(mode);
      thread::sleep(Duration::from_millis(4000));
    };
    if prev.len() == 0{
      prev = payload;
    } else {
      prev = write_db(payload, prev, &connection);
    }
  }
}

fn write_db(payload: Vec<Vec<f32>>, prev : Vec<Vec<f32>>, connection : &sqlite::Connection) -> Vec<Vec<f32>>{
  for index in 0..payload.len(){
    match payload[index][0] as i32{
      0 => if payload[index][1] != prev[index][1]{
        let general : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO general VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});", payload[index][1], payload[index][2], payload[index][3], payload[index][4], payload[index][5], payload[index][6], payload[index][7], payload[index][8], payload[index][9], payload[index][10], payload[index][11], payload[index][12], payload[index][13], payload[index][14],payload[index][15], payload[index][16], payload[index][17], payload[index][18], payload[index][19], payload[index][20], payload[index][21], payload[index][22], payload[index][23], payload[index][24], payload[index][25], payload[index][26], payload[index][27], payload[index][28], payload[index][29], payload[index][30], payload[index][31], payload[index][32], payload[index][33]));
        if general.is_ok(){
          print!("general packet recived, ");
        } else {
          print!("general packet err: {:?}, ", general.err().unwrap().to_string());
        }
      } 
      1 => if payload[index][1] != prev[index][1]{
        let sus : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO suspension VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});", payload[index][1], payload[index][2], payload[index][3], payload[index][4], payload[index][5], payload[index][6], payload[index][7], payload[index][8], payload[index][9], payload[index][10], payload[index][11], payload[index][12], payload[index][13], payload[index][14], payload[index][15], payload[index][16], payload[index][17], payload[index][18], payload[index][19], payload[index][20], payload[index][21], payload[index][22], payload[index][23]));
        if sus.is_ok() {
          print!("suspension packet recived, ");
        } else {
          print!("suspension packet err: {:?}, ", sus.err().unwrap().to_string());
        }
      }
      2 => if payload[index][1] != prev[index][1]{
        let damper : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO damper VALUES ({}, {}, {}, {}, {}, {},{}, {}, {});", payload[index][1], payload[index][2], payload[index][3], payload[index][4], payload[index][5], payload[index][6], payload[index][7], payload[index][8], payload[index][9]));
        if damper.is_ok(){
          print!("damper packet recived ");
        } else {
          print!("damper err: {:?} ", damper.err().unwrap().to_string());
        }
      }
      3 => if payload[index][1] != prev[index][1]{
        let drive : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO drive VALUES ({}, {}, {}, {}, {}, {}, {},{}, {}, {}, {}, {}, {}, {}, {}, {});", payload[index][1], payload[index][2], payload[index][3], payload[index][4], payload[index][5], payload[index][6], payload[index][7], payload[index][8], payload[index][9], payload[index][10], payload[index][11], payload[index][12], payload[index][13], payload[index][14], payload[index][15], payload[index][16]));
        if drive.is_ok(){
          print!("drive packet recived, ");
        } else {
          print!("drive err: {:?}, ", drive.err().unwrap().to_string());
        }
      }
      4 => if payload[index][1] != prev[index][1]{
        let slide : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO slide VALUES ({}, {}, {}, {}, {}, {}, {},{}, {}, {}, {}, {});", payload[index][1], payload[index][2], payload[index][3], payload[index][4], payload[index][5], payload[index][6], payload[index][7], payload[index][8], payload[index][9], payload[index][10], payload[index][11], payload[index][12]));
        if slide.is_ok(){
          print!("slide packet recived, ");
        } else {
          print!("slide err: {:?}, ", slide.err().unwrap().to_string());
        }
      }
      _ => println!("Invalid group"),
    }
  }
  println!("{:?}", payload);
  return payload.clone();
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
    let mut chosen : usize;
    loop{
      println!("portvec {:?}", port_names);
      let mut input = String::new();
      let _ = io::stdin().read_line(&mut input);
      let result : Result<usize, std::num::ParseIntError> = input.trim().parse();
      if result.is_ok(){
        chosen = result.unwrap();
        if chosen < port_names.len(){
          break;
        } else {
          println!("Invalid number, choice is 0 indexed");
        }
      } else {
        println!("Input is not a number");
      }
    }
    curr_port = ports[chosen].clone();
    println!("Using port {}", curr_port.port_name);

    let port = serialport::new(curr_port.port_name, BAUD)
      .timeout(Duration::from_millis(TIMEOUT))
      .flow_control(serialport::FlowControl::Hardware)
      .open_native();
  return Ok(port.unwrap())
  }
}

fn generate_fake_data(mode : i32) -> Vec<Vec<f32>> {
  let mut rng = rand::thread_rng();
  let mut fake_data = Vec::new();
  let mut packet_count = 1;
  match mode{
    1 | 3=> packet_count = 2,
    2 => packet_count = 9,
    4 => packet_count = 5,
    _ => packet_count = 1
  }
  for _ in 0..packet_count {
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

fn decode(mode : i32, buf : Vec<u8>) -> Vec<Vec<f32>>{
  let packet_count:usize;
  match mode{
    1 | 3 => packet_count = 2,
    2 => packet_count = 9,
    4 => packet_count = 5,
    _ => packet_count = 1
  }
  let mut vec_index = 0;
  let mut final_vec : Vec<Vec<f32>> = vec![vec![]; packet_count];
  let mut index: usize = 0;
  let mut decimal: bool = false;
  let mut negitive: bool = false;
  let mut decimal_counter: u32 = 0;
  //println!("{:?}", buf);
  for i in buf{
    if i as char == '\n'{
      if(vec_index + 1 == packet_count){
        break;
      }
      vec_index += 1;
      index = 0;
      decimal = false;
      negitive = false;
      decimal_counter = 0;
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