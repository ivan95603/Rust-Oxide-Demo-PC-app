use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box as Box_, Orientation,};

use std::io::{self, Write};
use std::time::Duration;

use std::str;

enum Message {
    UpdateObjectTemperatureLabel(String),
    UpdateAmbientTemperatureLabel(String),
    UpdatePressureLabel(String),
}


fn build_ui(application: &gtk::Application) {


    let window = ApplicationWindow::builder()
    .application(application)
    .title("Oxide Demo PC App")
    .default_width(350)
    .default_height(70)
    .build();

    let vbox = Box_::new(Orientation::Vertical, 0);


    let hboxObjectTemperature = Box_::new(Orientation::Horizontal, 5);

    let labelObjectTemperatureLabel = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Object temperature: ")) ;
    hboxObjectTemperature.append(&labelObjectTemperatureLabel);

    let labelObjectTemperatureData = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Object temperature DATA")) ;
    let labelObjectTemperatureData_clone = labelObjectTemperatureData.clone();
    hboxObjectTemperature.append(&labelObjectTemperatureData);

    vbox.append(&hboxObjectTemperature);



    let hboxAmbientTemperature = Box_::new(Orientation::Horizontal, 5);

    let labelAmbientTemperatureLabel = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Ambient temperature: ")) ;
    hboxAmbientTemperature.append(&labelAmbientTemperatureLabel);

    let labelAmbientTemperatureData = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Ambient temperature DATA")) ;
    let labelAmbientTemperatureData_clone = labelAmbientTemperatureData.clone();
    hboxAmbientTemperature.append(&labelAmbientTemperatureData);

    vbox.append(&hboxAmbientTemperature);


    let hboxPressure = Box_::new(Orientation::Horizontal, 5);

    let labelPressureLabel = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Pressure: ")) ;
    hboxPressure.append(&labelPressureLabel);

    let labelPressureData = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Pressure DATA")) ;
    let labelPressureData_clone = labelPressureData.clone();
    hboxPressure.append(&labelPressureData);

    vbox.append(&hboxPressure);

    window.set_child(Some(&vbox));




   // Create a new sender/receiver pair with default priority
   let (sender, receiver) = gtk::glib::MainContext::channel(gtk::glib::PRIORITY_DEFAULT);

   // Spawn the thread and move the sender in there
   std::thread::spawn(move || {

   let port_name = "/dev/ttyUSB0";
   let baud_rate = 115200 as u32;

   let port = serialport::new(port_name, baud_rate)
       .timeout(Duration::from_millis(10))
       .open();

   match port {
       Ok(mut port) => {
           let mut serial_buf: Vec<u8> = vec![0; 1000];
           println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
           loop {
               match port.read(serial_buf.as_mut_slice()) {
                   Ok(t) => {
                       let s = match str::from_utf8(&serial_buf[..t]) {
                           Ok(v) => v,
                           Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                       };

                       let split = s.split(":").collect::<Vec<&str>>();
                       let size = split.len();

                       match size {
                           2 => {
                                match split[0]{
                                    "OT" => {
                                        let _ = sender.send(Message::UpdateObjectTemperatureLabel(split[1].trim().to_owned()));
                                    },
                                    "AT" => {
                                        let _ = sender.send(Message::UpdateAmbientTemperatureLabel(split[1].trim().to_owned()));
                                    },
                                    //TODO: ADD PRESSURE
                                    _ => {
                                        println!("Data code not valid.");
                                    }
                                }
                           },
                           _ => {
                               println!("Arriving string size problem. Size: {}", size);
                           }
                       }
                   },
                   Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                   Err(e) => eprintln!("{:?}", e),
               }
           }
       }
       Err(e) => {
           eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
           ::std::process::exit(1);
       }
   }
   });


    // Attach the receiver to the default main context (None)
    // and on every message update the label accordingly.
    receiver.attach(None, move |msg| {
        match msg {
            Message::UpdateObjectTemperatureLabel(text) => {
                // println!("UpdateObjectTemperatureLabel {}", &text);
                labelObjectTemperatureData_clone.set_text(text.as_str());
            },
            Message::UpdateAmbientTemperatureLabel(text) => {
                // println!("UpdateAmbientTemperatureLabel {}", &text);
                labelAmbientTemperatureData_clone.set_text(text.as_str());
            },
            Message::UpdatePressureLabel(text) => {
                // println!("UpdatePressureLabel {}", &text);
                labelPressureData_clone.set_text(text.as_str());
            },
        }

        // Returning false here would close the receiver
        // and have senders fail
        gtk::glib::Continue(true)
    });

    window.show();
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}