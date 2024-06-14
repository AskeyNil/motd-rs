use std::time::Duration;

use humantime::format_duration;
use serde::Deserialize;
use termion::{color, style};

use crate::command::BetterCommand;
use crate::tools::color_to_rgb8;

use super::component::Component;

#[derive(Debug, Deserialize)]
pub struct System {
    title_command: String,
    title_color: String,
    #[serde(default)]
    show_up_time: bool,
}

impl Component for System {
    fn print(&self, _: usize) {
        let output = BetterCommand::new_with_bash()
            .arg(&self.title_command)
            .check_status_and_get_output_string()
            .expect(&format!(
                "system.title_command = \"{}\" failed.",
                self.title_command
            ));

        let color = color_to_rgb8(&self.title_color).expect(&format!(
            "system.title_color = \"{}\" is not supported.",
            self.title_color
        ));

        let color = color::Rgb(color.0, color.1, color.2);
        let color = color::Fg(color);
        let reset = style::Reset;

        print!("{color}{}{reset}", output.trim_end());

        let os_version = sysinfo::System::long_os_version().unwrap_or("".to_string());
        let kernel_version = sysinfo::System::kernel_version().unwrap_or("".to_string());
        let cpu_arch = sysinfo::System::cpu_arch().unwrap_or("".to_string());

        println!("");
        println!("");
        println!("  {os_version} {kernel_version} {cpu_arch}",);

        if self.show_up_time {
            let uptime = sysinfo::System::uptime();
            let uptime = Duration::from_secs(uptime);
            println!("  Up {}", format_duration(uptime).to_string());
        }
        println!("");
    }
}
