pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("[{}]", self.label);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        let last = self.options.len() - 1;
        for (index, option) in self.options.iter().enumerate() {
            let margin = match index {
                0 => "┏",
                l if l == last => "┗",
                _ => "┣"
            };
            println!("{}{}", margin, option);
        }
    }
}

fn main() {
    let s = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 20,
                options: vec![
                    String::from("Bleh"),
                    String::from("Bleu"),
                    String::from("Blih"),
                ],
            }),
            Box::new(Button {
                width: 32,
                height: 10,
                label: String::from("OK"),
            }),
            // Box::new(String::from("Hi!")),
            // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `std::string::String`
        ]
    };

    s.run();
}
