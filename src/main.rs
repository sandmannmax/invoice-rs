extern crate azul_text_layout;
extern crate chrono;
extern crate genpdf;

use chrono::naive::NaiveDate;
use genpdf::{elements, style};
use std::fmt;

struct Instance {
    name: String,
    street: String,
    city: String,
}

impl Instance {
    fn get_layout(&self) -> elements::PaddedElement<elements::LinearLayout> {
        let mut layout = elements::LinearLayout::vertical();
        layout.push(elements::Paragraph::new(&self.name));
        layout.push(elements::Paragraph::new(&self.street));
        layout.push(elements::Paragraph::new(&self.city));
        elements::PaddedElement::new(layout, genpdf::Margins::trbl(10, 0, 10, 0))
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::clone::Clone for Instance {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            street: self.street.clone(),
            city: self.city.clone(),
        }
    }
}

struct Position {
    name: String,
    quantity: f32,
    unit_price: f32,
}

impl Position {
    fn calculate_full_price(&self) -> f32 {
        self.quantity * self.unit_price
    }
}

struct Invoice {
    sender: Instance,
    receiver: Instance,
    invoice_id: i32,
    invoice_date: NaiveDate,
    output_date: NaiveDate,
    positions: Vec<Position>,
}

impl Invoice {
    fn pdf(&self, name: &str) {
        let font_family = genpdf::fonts::from_files("data/fonts/Cabin", "Cabin", None).unwrap();
        let mut doc = genpdf::Document::new(font_family);
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);
        doc.set_title("Invoice");
        doc.push(elements::StyledElement::new(
            elements::Text::new("Invoice"),
            style::Effect::Bold,
        ));
        doc.push(self.sender.get_layout());
        doc.push(self.receiver.get_layout());
        doc.render_to_file(format!("target/{}", name))
            .expect("Failed to write PDF file");
    }
}

fn main() {
    let inst = Instance {
        name: String::from("Company GmbH"),
        street: String::from("Companystreet 1"),
        city: String::from("12345 Companycity"),
    };
    let invoice = Invoice {
        sender: inst.clone(),
        receiver: inst.clone(),
        invoice_id: 1,
        invoice_date: NaiveDate::from_ymd_opt(2022, 11, 22).unwrap(),
        output_date: NaiveDate::from_ymd_opt(2022, 12, 6).unwrap(),
        positions: vec![Position {
            name: String::from("Book"),
            amount: 1.,
            price: 10.,
        }],
    };
    invoice.pdf("invoice.pdf");
}
