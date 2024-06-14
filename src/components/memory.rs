use std::vec;

use comfy_table::{presets::NOTHING, Cell, Row, Table};
use serde::Deserialize;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use unicode_width::UnicodeWidthStr;

use crate::tools::{process_str, ByteStr};

use super::component::Component;

fn true_value() -> bool {
    true
}

fn red_value() -> String {
    "red".to_string()
}

fn green_value() -> String {
    "green".to_string()
}

fn threshold_value() -> f64 {
    0.8
}

#[derive(Debug, Deserialize)]
pub struct Memory {
    #[serde(default = "true_value")]
    show_ram: bool,

    #[serde(default = "true_value")]
    show_swap: bool,

    #[serde(default = "green_value")]
    default_color: String,

    #[serde(default = "red_value")]
    warning_color: String,

    #[serde(default = "threshold_value")]
    warning_threshold: f64,
}

impl Memory {
    pub fn table(&self) -> Table {
        let mut table: Table = Table::new();

        if !self.show_ram && self.show_swap {
            return table;
        }

        table
            .load_preset(NOTHING)
            .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);

        let sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        if self.show_ram {
            let ram_title = Cell::new("RAM:");
            let ram_value = Cell::new(format!(
                "{} / {} ({:.1}%)",
                sys.used_memory().byte_str(),
                sys.total_memory().byte_str(),
                sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0
            ))
            .set_alignment(comfy_table::CellAlignment::Right);

            let mut ram_row = Row::new();
            ram_row.add_cell(ram_title);
            ram_row.add_cell(ram_value);
            table.add_row(ram_row);
        }

        if self.show_swap {
            let ram_title = Cell::new("Swap:");
            let ram_value = Cell::new(format!(
                "{} / {} ({:.1}%)",
                sys.used_swap().byte_str(),
                sys.total_swap().byte_str(),
                sys.used_swap() as f64 / sys.total_swap() as f64 * 100.0
            ))
            .set_alignment(comfy_table::CellAlignment::Right);
            let mut ram_row = Row::new();
            ram_row.add_cell(ram_title);
            ram_row.add_cell(ram_value);
            table.add_row(ram_row);
        }

        table
    }

    fn get_per(&self) -> Vec<f64> {
        let sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        let mem = sys.used_memory() as f64 / sys.total_memory() as f64;
        let swap = sys.used_swap() as f64 / sys.total_swap() as f64;

        let vec = match (self.show_ram, self.show_swap) {
            (true, true) => vec![mem, swap],
            (_, true) => vec![swap],
            (true, _) => vec![mem],
            (_, _) => vec![],
        };
        vec
    }
}

impl Component for Memory {
    fn width(&self) -> usize {
        return 0;
    }

    fn print(&self, width: usize) {
        let mut table: Table = self.table();
        table.set_width(width as u16);

        let width = table.lines().map(|s| s.width()).max().unwrap();
        let per = self.get_per();
        println!("Memory:");
        for (index, line) in table.lines().enumerate() {
            println!("  {}", line);
            let process = process_str(
                width,
                per[index],
                &self.default_color,
                &self.warning_color,
                self.warning_threshold,
            );
            println!("  {process}");
        }
        println!("");
    }
}
