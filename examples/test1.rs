use std::io::Write;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use virtualbox_rs::{FramebufferEvent, Session, VirtualBox};
use virtualbox_rs::enums::{BitmapFormat, SessionType};

#[tokio::main]
async fn main() {
    env_logger::init();

    let vbox = VirtualBox::init().unwrap();
    let mut session = Session::init().unwrap();
    let machine = vbox.
            find_machines("winxp").unwrap();
    
    machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    
    let console = session.get_console().unwrap();
    
    let mut display = console.get_display().unwrap();
    let buffer = display.get_framebuffer(BitmapFormat::JPEG, 30).unwrap();
    // let buffer = display.query_framebuffer(0).unwrap();
    for i in 0..10 {
        // sleep(Duration::from_millis(33)).await;
        let start_time = Instant::now();

        let image = buffer.get_image(true, 1000).await.unwrap();
        println!("Get_image: {:.6}", start_time.elapsed().as_secs_f64());
        match image {
            FramebufferEvent::FramebufferImage(image) => {
                let mut file = std::fs::File::create(format!("img/output{}.png", i)).unwrap();
                // let img = framebuffer_to_jpeg(&image).unwrap();
                // file.write_all(&img).unwrap();
                file.write_all(&image.data).unwrap();
            }
            FramebufferEvent::ChangeResolution(width, height) => {
                println!("ChangeResolution: {}x{}", width, height);
            }
            FramebufferEvent::None => {
                println!("None");
            }
        }
    }
}
