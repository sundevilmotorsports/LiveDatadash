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
use charming::{
    component::{Axis, Legend}, element::{AxisType, ItemStyle}, renderer::{self, HtmlRenderer}, series::{Line, Pie, PieRoseType}, Chart
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
async fn get_chart() -> String{
    let chart = Chart::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260])
    );

    let mut renderer = renderer::HtmlRenderer::new("testing", 800, 800);
    println!("rendered");
    return renderer.render(&chart).unwrap();
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
    
    //let temp = renderer.save(&chart, "C:/Users/kaden/Desktop/temp/SAE/temp.html");
    //println!("saved {:?}", temp.err());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports, app_handle, update_port, get_chart])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

