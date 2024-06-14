use comfy_table::{presets::NOTHING, Cell, Row, Table};
use serde::Deserialize;
use sysinfo::Networks;
use termion::color;

use crate::tools::ByteStr;

use super::component::Component;

#[derive(Debug, Deserialize)]
pub struct Network {
    #[serde(default)]
    interfaces: Vec<String>,
    #[serde(default)]
    show_mac: bool,
    #[serde(default)]
    show_flow: bool,
}

impl Component for Network {
    fn print(&self, _: usize) {
        if !self.show_flow && !self.show_mac {
            return;
        }
        println!("Network:");
        let mut table = Table::new();
        table
            .load_preset(NOTHING)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

        let networks = Networks::new_with_refreshed_list();
        let red = color::Fg(color::Red);
        let green = color::Fg(color::Green);
        let reset = color::Fg(color::Reset);

        for interface in self.interfaces.iter() {
            let data = networks.get(interface);
            if data.is_none() {
                continue;
            }
            let mut row = Row::new();
            if self.show_flow {
                let up_icon = format!("({green}▼{red}▲{reset}");
                let up = format!("{}", data.unwrap().total_received().byte_str());
                let down_icon = format!("{red}▼{green}▲{reset}");
                let down = format!("{})", data.unwrap().total_transmitted().byte_str());

                row.add_cell(Cell::new(interface.trim()));
                row.add_cell(Cell::new(up_icon));
                row.add_cell(Cell::new(up).set_alignment(comfy_table::CellAlignment::Right));
                row.add_cell(Cell::new("/"));
                row.add_cell(Cell::new(down_icon));
                row.add_cell(Cell::new(down).set_alignment(comfy_table::CellAlignment::Right));
            } else {
                println!("  {}", interface)
            }
            if self.show_mac {
                let mac = format!("mac: {}", data.unwrap().mac_address());
                row.add_cell(Cell::new(mac));
            }
            table.add_row(row);
        }
        for (i, c) in table.column_iter_mut().enumerate() {
            match i {
                1..=4 => c.set_padding((1, 0)),
                _ => c,
            };
        }
        println!("{table}");
        println!("");
    }
}
