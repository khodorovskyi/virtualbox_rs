use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use log::error;
use virtualbox_rs::{Framebuffer, Session, VboxError, VirtualBox, VirtualBoxClient};
use virtualbox_rs::enums::{BitmapFormat, FrontEndName, SessionType};

fn main() {
    env_logger::init();

    // let vbox = VirtualBox::init().unwrap();
    // let mut session = Session::init().unwrap();
    // let machine = vbox.
    //         find_machines("FreeBSD-14.0-RELEASE").unwrap();
    // machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    // let console = session.get_console().unwrap();
    // 
    // let mut display = console.get_display().unwrap();
    // 
    // let framebuffer = display.query_framebuffer(0).unwrap();
    // println!("{:#?}", framebuffer);
    // display.take_screen_shot_to_array(
    //     0, 
    //     framebuffer.get_width().unwrap(),
    //     framebuffer.get_height().unwrap(),
    //     BitmapFormat::PNG).unwrap();
    // // return;
    // 
    println!("__________________________________");
    // 
    // let m = VirtualBox::init().unwrap().get_machines().unwrap();
    // println!("machines: {:#?}", m);
    // 
    
    let vbox = VirtualBox::init().unwrap();
    let mut session = Session::init().unwrap();
    let machine = vbox.
            find_machines("ubuntu_24_04").unwrap();
    
    machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    
    let console = session.get_console().unwrap();
    
    let mut display = console.get_display().unwrap();
    let buffer = Framebuffer::init();
    display.invalidate_and_update().unwrap();
    display.attach_framebuffer(0, &buffer).unwrap_or_else(|error|{
        error!("Error attaching framebuffer: {:?}", error);
            ""
        });
    // 
    let framebuffer = display.query_framebuffer(0).unwrap();
    display.viewport_changed(0, 0, 0, framebuffer.get_width().unwrap(), framebuffer.get_height().unwrap()).unwrap();
    let framebuffer = display.query_framebuffer(0).unwrap();
    println!("{:#?}", framebuffer);
    // sleep(Duration::from_secs(5));
    
    
    match framebuffer.get_image() {
        Ok(image) => {
            let mut file = File::create("output.png").unwrap();
            file.write_all(&image).unwrap();
        }
        Err(error) => {error!("Error getting image: {:?}", error); }
    };
    let framebuffer = display.query_framebuffer(0).unwrap();
    sleep(Duration::from_secs(1));

    
    
    match framebuffer.get_image() {
        Ok(image) => {
            let mut file = File::create("output.png").unwrap();
            file.write_all(&image).unwrap();
        }
        Err(error) => {error!("Error getting image: {:?}", error); }
    };
    let framebuffer = display.query_framebuffer(0).unwrap();
    sleep(Duration::from_secs(1));

    
    
    match framebuffer.get_image() {
        Ok(image) => {
            let mut file = File::create("output.png").unwrap();
            file.write_all(&image).unwrap();
        }
        Err(error) => {error!("Error getting image: {:?}", error); }
    };
    let framebuffer = display.query_framebuffer(0).unwrap();
    sleep(Duration::from_secs(1));

    
    
    match framebuffer.get_image() {
        Ok(image) => {
            let mut file = File::create("output.png").unwrap();
            file.write_all(&image).unwrap();
        }
        Err(error) => {error!("Error getting image: {:?}", error); }
    };
    let framebuffer = display.query_framebuffer(0).unwrap();
    sleep(Duration::from_secs(1));


    // match framebuffer.get_image() {
    //     Ok(image) => {
    //         let mut file = File::create("output.png").unwrap();
    //         file.write_all(image).unwrap();
    //     }
    //     Err(error) => {error!("Error getting image: {:?}", error); }
    // };
    // let framebuffer = display.query_framebuffer(0).unwrap();
    // sleep(Duration::from_secs(1));
    // 
    // match framebuffer.get_image() {
    //     Ok(image) => {
    //         let mut file = File::create("output.png").unwrap();
    //         file.write_all(image).unwrap();
    //     }
    //     Err(error) => {error!("Error getting image: {:?}", error); }
    // };
    // let framebuffer = display.query_framebuffer(0).unwrap();
    // sleep(Duration::from_secs(1));
    // match framebuffer.get_image() {
    //     Ok(image) => {
    //         let mut file = File::create("output.png").unwrap();
    //         file.write_all(image).unwrap();
    //     }
    //     Err(error) => {error!("Error getting image: {:?}", error); }
    // };
    // let framebuffer = display.query_framebuffer(0).unwrap();
    // sleep(Duration::from_secs(1));
    // match framebuffer.get_image() {
    //     Ok(image) => {
    //         let mut file = File::create("output.png").unwrap();
    //         file.write_all(image).unwrap();
    //     }
    //     Err(error) => {error!("Error getting image: {:?}", error); }
    // };
    // let framebuffer = display.query_framebuffer(0).unwrap();
    // sleep(Duration::from_secs(1));
    // match framebuffer.get_image() {
    //     Ok(image) => {
    //         let mut file = File::create("output.png").unwrap();
    //         file.write_all(image).unwrap();
    //     }
    //     Err(error) => {error!("Error getting image: {:?}", error); }
    // };
 
}