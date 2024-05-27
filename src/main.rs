use std::collections::HashMap;
use eframe::{run_native, App};
use rand::seq::SliceRandom;

const PREFIX_IN:u8 = 0x03;
const PREFIX_OUT:u8 = 0x1F;

fn katharevousopoisis(input: &str, vowels: &HashMap< &u8, &Vec<u8>> )-> String{
    input.chars()
        .map(|c| if ((c as u32) >> 8) as u8 == PREFIX_IN && vowels.contains_key(&(c as u8)) { 
            char::from_u32(
                ((PREFIX_OUT as u32) << 8) | *(vowels.get( &(c as u8)).unwrap().choose(&mut rand::thread_rng()).unwrap()) as u32
            ).unwrap() 
            } 
            else {c} 
        )
        .collect()
}

fn main() {
    
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();

    println!("{}", katharevousopoisis(&line, &vowels_map));
    
/*
    let native_options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
           .with_inner_size([400.0, 300.0])
           .with_min_inner_size([300.0, 220.0])
           //.with_icon()
           ,
        ..Default::default()
    };

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
    )
  */

    //let app = SpamotonicSystem {vowels_map};
    let native_options = eframe::NativeOptions::default();
    //Text::run(Settings::default())
    run_native("Σπαμοτονικό Σύστημα", native_options, Box::new(|cc| Box::new(SpamotonicSystem::new(cc))));
}
impl SpamotonicSystem{
    fn new(cc: &eframe::CreationContext<'_>) -> Self{
        let mut vowels_map = HashMap::new();
        let a_in: Vec<u8> = vec![0xb1, 0xac];
        let e_in: Vec<u8> = vec![0xb5, 0xad];
        let h_in: Vec<u8> = vec![0xb7, 0xae];
        let i_in: Vec<u8> = vec![0xb9, 0xaf, 0xca, 0x90];
        let o_in: Vec<u8> = vec![0xbf, 0xcc];
        let y_in: Vec<u8> = vec![0xc5, 0xcd, 0xcb, 0xb0];
        let v_in: Vec<u8> = vec![0xc9, 0xce];
        let r_in: Vec<u8> = vec![0xc1];
        
        let a_out: Vec<u8> = (0x00..=0x07).chain(0x70..=0x71).chain(0x80..=0x87).chain(0xb0..=0xb4).chain(0xb6..=0xb7).collect();
        let e_out: Vec<u8> = (0x10..=0x15).chain(0x72..=0x73).collect();
        let h_out: Vec<u8> = (0x20..=0x27).chain(0x74..=0x75).chain(0x90..=0x97).chain(0xc2..=0xc4).chain(0xc6..=0xc7).collect();
        let i_out: Vec<u8> = (0x30..=0x37).chain(0x76..=0x77).chain(0xd0..=0xd3).chain(0xd6..=0xd7).collect();
        let o_out: Vec<u8> = (0x40..=0x45).chain(0x78..=0x79).collect();
        let y_out: Vec<u8> = (0x50..=0x57).chain(0x7a..=0x7b).chain(0xe0..=0xe2).chain(0xe6..=0xe7).collect();
        let v_out: Vec<u8> = (0x60..=0x67).chain(0x7c..=0x7d).chain(0xa0..=0xa7).chain(0xf2..=0xf4).chain(0xf6..=0xf7).collect(); 
        
        
        let a_cin: Vec<u8> = vec![0x91, 0x86];
        let e_cin: Vec<u8> = vec![0x95, 0x88];
        let h_cin: Vec<u8> = vec![0x97, 0x89];
        let i_cin: Vec<u8> = vec![0x99, 0x8a, 0xaa];
        let o_cin: Vec<u8> = vec![0x9f, 0x8c];
        let y_cin: Vec<u8> = vec![0xa5, 0x8e, 0xab];
        let v_cin: Vec<u8> = vec![0xa9, 0x8f];
        let r_cin: Vec<u8> = vec![0xa1];
        
        let a_cout: Vec<u8> = (0x08..=0x0f).chain(0x88..=0x8f).chain(0xb8..=0xbc).collect();
            let e_cout: Vec<u8> = (0x18..=0x1d).chain(0xc8..=0xc9).collect();
        let h_cout: Vec<u8> = (0x28..=0x2f).chain(0x98..=0x9f).chain(0xca..=0xcc).collect();
        let i_cout: Vec<u8> = (0x38..=0x3f).chain(0xd8..=0xdb).collect();
        let o_cout: Vec<u8> = (0x48..=0x4d).chain(0xf8..=0xf9).collect();
        let y_cout: Vec<u8> = [0x59, 0x5b, 0x5d, 0x5f].into_iter().chain(0xe8..=0xeb).collect();
        let v_cout: Vec<u8> = (0x68..=0x6f).chain(0xa8..=0xaf).chain(0xfa..=0xfc).collect(); 

        let r_out: Vec<u8> = (0xe4..=0xe5).collect();
        let r_cout: Vec<u8> = vec![0xec_u8];
        
        a_in.iter().for_each(|item|  { vowels_map.insert(item, &a_out); });
        e_in.iter().for_each(|item|  { vowels_map.insert(item, &e_out); });
        h_in.iter().for_each(|item|  { vowels_map.insert(item, &h_out); });
        i_in.iter().for_each(|item|  { vowels_map.insert(item, &i_out); });
        o_in.iter().for_each(|item|  { vowels_map.insert(item, &o_out); });
        y_in.iter().for_each(|item|  { vowels_map.insert(item, &y_out); });
        v_in.iter().for_each(|item|  { vowels_map.insert(item, &v_out); });
        r_in.iter().for_each(|item|  { vowels_map.insert(item, &r_out); });
        
        a_cin.iter().for_each(|item|  { vowels_map.insert(item, &a_cout); });
        e_cin.iter().for_each(|item|  { vowels_map.insert(item, &e_cout); });
        h_cin.iter().for_each(|item|  { vowels_map.insert(item, &h_cout); });
        i_cin.iter().for_each(|item|  { vowels_map.insert(item, &i_cout); });
        o_cin.iter().for_each(|item|  { vowels_map.insert(item, &o_cout); });
        y_cin.iter().for_each(|item|  { vowels_map.insert(item, &y_cout); });
        v_cin.iter().for_each(|item|  { vowels_map.insert(item, &v_cout); });
        r_cin.iter().for_each(|item|  { vowels_map.insert(item, &r_cout); });

        let spamotonic_system = Self{
            vowels_map
        };
        spamotonic_system
    }
}
#[derive(Default)]
struct SpamotonicSystem<'a>{
    vowels_map: HashMap<&'a u8, &'a Vec<u8>>
}
impl App for SpamotonicSystem{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Σπαμοτονικό Σύστημα");
        });
    }
}
