// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serial_manager;
use std::{sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex
}, time::Duration};
use serialport::{
    COMPort, 
    SerialPort,
    SerialPortInfo,
    SerialPortType,
    UsbPortInfo,
    Error
};
use std::thread;
use tauri::Manager;

static mut PORT : String = String::new();

pub struct AppData(Mutex<Data>);

pub struct Data{
    packet : Vec<Vec<i32>>,
    is_listening : bool
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[derive(Clone, serde::Serialize)]
struct LeanPort {
    //brand_name : SerialPortType,
    port_name : String,
    port_exists : bool
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_ports() -> Vec<LeanPort> {
    let ports : Vec<SerialPortInfo> = serial_manager::get_ports();
    let mut lean : Vec<LeanPort> = vec![];
    for i in ports {
        lean.push(LeanPort {port_name : i.port_name, port_exists : i.port_type != SerialPortType::Unknown});
    };
    lean.reverse();
    return lean
    
}


#[tauri::command]
async fn app_handle(app: tauri::AppHandle) -> bool{
    let app_clone = app.clone();
    let state = app_clone.state::<AppData>();
    let mut state_guard = state.0.lock().unwrap(); 
    // if state_guard.is_reading {

    // }
    return false;
}

#[tauri::command]
async fn update_port(name : String){
    //let temp : Mutex<String> = Mutex::new(name);
    println!("inside update_port {}", name);
    unsafe {
        PORT = name;
    }
}

pub fn start_thread(app : tauri::AppHandle){
    let mut serial_buf: Vec<i32> = vec![];
    thread::spawn( move || {
        unsafe{
            let curr_port = serialport::new(PORT.clone(), 115200)
                .timeout(Duration::from_secs(5))
                .open_native()
                .expect("Failed to open port");
        }
        // is_thread_open.store(true, Ordering::Relaxed);
        // while is_thread_open.load(Ordering::Relaxed){   
        // };

    });
}

fn main() {
    println!("begin main");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports, app_handle, update_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
