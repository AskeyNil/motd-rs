use super::component::Component;
use crate::tools::{process_str, ByteStr};

use comfy_table::Table;
use comfy_table::*;
use presets::NOTHING;
use serde::Deserialize;
use std::collections::HashMap;
use sysinfo::Disks;
use unicode_width::UnicodeWidthStr;

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
pub struct Disk {
    #[serde(default = "green_value")]
    default_color: String,

    #[serde(default = "red_value")]
    warning_color: String,

    #[serde(default = "threshold_value")]
    warning_threshold: f64,

    #[serde(default)]
    mounts: HashMap<String, String>,
}

impl Disk {
    fn table(&self, items: &[DiskItem]) -> Table {
        let mut table = Table::new();
        table
            .load_preset(NOTHING)
            .set_content_arrangement(ContentArrangement::Dynamic);

        let titles = vec!["name", "fs", "type", "mount", "used", "total", "avail"];
        let mut header = Row::new();
        for title in titles {
            header.add_cell(
                Cell::new(title)
                    .set_alignment(CellAlignment::Center)
                    .set_delimiter(' '),
            );
        }
        table.add_row(header);

        for item in items {
            table.add_row(item.row());
        }

        table
    }

    fn get_items(&self) -> Vec<DiskItem> {
        let mut items = Vec::new();
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            let mount = disk.mount_point().to_str().unwrap_or("");
            if self.mounts.get(mount).is_none() {
                continue;
            }
            let name = self.mounts.get(mount).unwrap().to_string();
            let item = DiskItem::new(name, disk);
            items.push(item);
        }
        items
    }
}

struct DiskItem {
    name: String,
    fs: String,
    kind: String,
    mount: String,
    used: String,
    total: String,
    avail: String,
    per: f64,
}

impl DiskItem {
    fn new(name: String, disk: &sysinfo::Disk) -> DiskItem {
        let name = name;
        let fs = disk.file_system().to_str().unwrap_or("").to_string();
        let kind = disk.kind().to_string();
        let mount = disk.mount_point().to_str().unwrap_or("").to_string();
        let used = (disk.total_space() - disk.available_space()).byte_str();
        let total = disk.total_space().byte_str();
        let avail = disk.available_space().byte_str();
        let per = 1.0 - disk.available_space() as f64 / disk.total_space() as f64;
        DiskItem {
            name,
            fs,
            kind,
            mount,
            used,
            total,
            avail,
            per,
        }
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.fs.clone(),
            self.kind.clone(),
            self.mount.clone(),
            self.used.clone(),
            self.total.clone(),
            self.avail.clone(),
        ]
    }
}

impl Component for Disk {
    fn width(&self) -> usize {
        let items = self.get_items();
        let table = self.table(&items);
        table.lines().map(|s| s.width()).max().unwrap()
    }

    fn print(&self, width: usize) {
        if self.mounts.is_empty() {
            return;
        }
        let items = self.get_items();
        let mut table = self.table(&items);
        table
            .set_width(width as u16)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth);

        let width = table.lines().map(|s| s.width()).max().unwrap();

        println!("Disk:");
        for (index, line) in table.lines().enumerate() {
            println!("  {}", line);
            if index > 0 {
                let process = process_str(
                    width,
                    items[index - 1].per,
                    &self.default_color,
                    &self.warning_color,
                    self.warning_threshold,
                );
                println!("  {process}");
            }
        }
        println!("");
    }
}
