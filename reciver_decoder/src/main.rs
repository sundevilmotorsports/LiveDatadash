use std::cmp::max;
use std::time::Duration;
use std::vec;
use serialport::SerialPort;
use serialport::SerialPortInfo;
use serialport::SerialPortType;
use rand::Rng;
use std::thread;

use std::io;
use std::io::Read;

const BAUD : u32 = 9600;
const TIMEOUT : u64 = 1000;
const PACKET_COUNT : usize = 3;

fn main() {
  let connection = sqlite::open("../server/ldd.db").unwrap();
  let init = "
        DROP TABLE IF EXISTS imu;
        DROP TABLE IF EXISTS wheel;
        DROP TABLE IF EXISTS datalog;
        DROP TABLE IF EXISTS ack;

        CREATE TABLE imu(
            id, 
            timestamp,
            x_acceleration, 
            y_acceleration, 
            z_acceleration, 
            x_gyro, 
            y_gyro, 
            z_gyro
        );

        CREATE TABLE wheel(
            id, 
            timestamp, 
            fl_wheel_speed, 
            fl_brake_temp, 
            fl_ambiant_temp,
            fr_wheel_speed, 
            fr_brake_temp, 
            fr_ambiant_temp,
            rl_wheel_speed, 
            rl_brake_temp, 
            rl_ambiant_temp,
            rr_wheel_speed, 
            rr_brake_temp, 
            rr_ambiant_temp
        );

        CREATE TABLE datalog(
            id INTEGER, 
            timestamp,
            drs, 
            steering_angle, 
            throttle_input,
            front_brake_pressure, 
            rear_brake_pressure,
            gps_lattitude, 
            gps_longitude, 
            battery_voltage, 
            daq_current_draw
        );

        CREATE TABLE ack(
            id, 
            timestamp
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
  /* 
  if port_result.is_ok() {
    port = port_result.unwrap();
    println!("port val: {:?}", port);
    real_data = true;
    // My only concern is that the serial port should queue data, if not big bruh moment
  } else {
    println!("No ports available. Generating fake data.");
  }
  println!("entering loop");
  */
  loop {
    let temp: Vec<Vec<f32>>;
    if let Some(ref mut port) = port {
      if port.bytes_to_read().unwrap() > 21 {
        println!("reading");
        let read : Result<usize, io::Error> = port.read(&mut serial_buf);
        println!("past read");
        if read.is_ok() {
          temp = decode(serial_buf.clone());
          println!("past decode");
          println!("buffer value: {serial_buf:?}");
        } else {
          println!("error: {:?}", read.err());
          continue;
        }
      } else {
        // In the event that the read is longer than expected i.e it is wrong and invalid
        println!("Current read from port is not valid, too long");
        continue;
      }
    } else {
      temp = generate_fake_data();
      thread::sleep(Duration::from_millis(4000));
    };
    prev = write_db(temp, prev, &connection);
  }
}

fn write_db(temp: Vec<Vec<f32>>, mut prev : Vec<Vec<f32>>, connection : &sqlite::Connection) -> Vec<Vec<f32>>{
  if temp[0][1] != prev[0][1]{
    prev[0] = temp[0].clone();
    let imu : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO imu VALUES (1, {}, {}, {}, {}, {}, {}, {});", temp[0][1], temp[0][2], temp[0][3], temp[0][4], temp[0][5], temp[0][6], temp[0][7]));
    print!("imu packet sent: {} ", imu.is_ok());
  } 
  if temp[1][1] != prev[1][1]{
    prev[1] = temp[1].clone();
    let wheel : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO wheel VALUES (2, {}, {}, {}, {}, {}, {}, {},{}, {}, {}, {}, {}, {});", temp[1][1], temp[1][2], temp[1][3], temp[1][4], temp[1][5], temp[1][6], temp[1][7], temp[1][8], temp[1][9], temp[1][10], temp[1][11], temp[1][12], temp[1][13]));
    print!("wheel packet sent: {} ", wheel.is_ok());
  }
  if temp[2][1] != prev[2][1]{
    prev[2] = temp[2].clone();
    let datalog : Result<(), sqlite::Error> = connection.execute(format!("INSERT INTO datalog VALUES (3, {}, {}, {}, {}, {}, {}, {},{}, {}, {});", temp[2][1], temp[2][2], temp[2][3], temp[2][4], temp[2][5], temp[2][6], temp[2][7], temp[2][8], temp[2][9], temp[2][10]));
    println!("datalog packet sent: {} ", datalog.is_ok());
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
          let value = rng.gen::<f32>();
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