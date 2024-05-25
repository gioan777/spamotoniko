use std::collections::HashMap;
use iced::{Element, Sandbox, Settings};

#[derive(Default)]
struct Text {
    text_input: String,
    text_output: String
}

#[derive(Debug, Clone, Copy)]
pub enum Message{
    Input
}

//use iced::widget::{shader::wgpu::core::device::SHADER_STAGE_COUNT, text_editor, Column};

impl Sandbox for Text{
    type Message = Message;

    fn new() -> Text {
        Text { text_input: "val".to_string(), text_output: "lmaoing".to_string() }
    }

    fn title(&self)->String {
        String::from("Σπαμοτονικό Σύστημα")
    }

    fn update(&mut self, _message: Self::Message){
        //LMAOING AT YOURE LIFE
    }

    fn view(&self) -> Element<Self::Message>{
        "LMAOING".into() 
    }
}

fn katharevousopoisis(input: &str, _vowels: &HashMap<Vec<u8>, Vec<u8>> )-> String{
    input.chars()
        //.map(|c| *( vowels.get( 0x03<<16 & (&c as u8)).unwrap_or(&c)))
        .collect()
} 
fn main() -> iced::Result {
    let mut vowels_map = HashMap::new();
    const PREFIX_IN:u64 = 0x03;
    const PREFIX_OUT:u64 = 0x1F;

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
  
    vowels_map.insert(a_in, a_out);
    vowels_map.insert(e_in, e_out);
    vowels_map.insert(h_in, h_out);
    vowels_map.insert(i_in, i_out);
    vowels_map.insert(o_in, o_out);
    vowels_map.insert(y_in, y_out);
    vowels_map.insert(v_in, v_out);
    vowels_map.insert(r_in, r_out);
 
    vowels_map.insert(a_cin, a_cout);
    vowels_map.insert(e_cin, e_cout);
    vowels_map.insert(h_cin, h_cout);
    vowels_map.insert(i_cin, i_cout);
    vowels_map.insert(o_cin, o_cout);
    vowels_map.insert(y_cin, y_cout);
    vowels_map.insert(v_cin, v_cout);
    vowels_map.insert(r_cin, r_cout);

    Text::run(Settings::default())
}
