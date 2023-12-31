// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate systemstat;
use serde::{Deserialize, Serialize};

use sysinfo::SystemExt;
use std::vec;
use systemstat::{saturating_sub_bytes, ByteSize, Duration, Platform, System};
use tauri::{Window, Manager};

#[derive(Serialize, Deserialize)]
struct CPUMemInfo {
    free: String,
    usage: String,
    total: String,
    free_size: u64,
    total_size: u64
}

#[derive(Serialize, Deserialize)]
struct SystemInfo {
    cpu_load: Vec<f32>,
    cpu_load_avg: f32,
    mem: CPUMemInfo,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn init_process(window: Window) -> Result<String, String> {

    let mut sys = sysinfo::System::new_all();
    let label = window.label().to_string();

    std::thread::spawn(move || {
        loop {
            sys.refresh_all();
            std::thread::sleep(Duration::from_millis(500));
            window.emit("system-stats", &sys).unwrap();
        }
    });



    Ok(String::from(label))

}

#[tauri::command]
async fn calculate_mem() -> String {
    let sys = System::new();

    let cpus_load_delayed = sys.cpu_load().expect("Faield to load cpus");
    let cpu_load_delayed = sys.cpu_load_aggregate().expect("failed to load cpu.");
    std::thread::sleep(Duration::from_secs(1));
    let cpus_load = cpus_load_delayed.done().expect("Failed to get cpus load");
    let cpu_load_avg = cpu_load_delayed.done().expect("Failed to get cpu avg load");

    // Cpu lload starts her
    let mut system_info = SystemInfo {
        cpu_load: Vec::new(),
        cpu_load_avg: 0.0f32,
        mem: CPUMemInfo {
            free: String::new(),
            usage: String::new(),
            total: String::new(),
            free_size: 0u64,
            total_size: 0u64
        },
    };

    for cpu_load in cpus_load {
        system_info.cpu_load.push(cpu_load.user);
    }

    system_info.cpu_load_avg = cpu_load_avg.user;

    // Memory loading info starts her
    let sys_mem = sys.memory().expect("failed to get system memory");
    let one_gig = ByteSize::gib(1).as_u64();
    let total_mem_size = ByteSize::b(sys_mem.total.as_u64() - one_gig);
    let free_mem_size = ByteSize::b(sys_mem.free.as_u64() - one_gig);

    system_info.mem.free = free_mem_size.to_string_as(false);
    system_info.mem.total = (total_mem_size).to_string_as(false);
    system_info.mem.usage = saturating_sub_bytes(sys_mem.total, sys_mem.free).to_string_as(false);
    system_info.mem.free_size = saturating_sub_bytes(sys_mem.total, sys_mem.free).as_u64();
    system_info.mem.total_size = total_mem_size.as_u64();

    //system_info.mem.free = saturating_sub_bytes(, r);

    //Network stuff here
    let sys_info = serde_json::to_string(&system_info).expect("failed to get string");

    sys_info
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

  
            let id = main_window.listen("system-stats", |event| {
                println!("window info is {:?}", event.payload())
            });

            main_window.unlisten(id);

            Ok(())

        })
        .invoke_handler(tauri::generate_handler![greet, calculate_mem, init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
