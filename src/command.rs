use crate::ui::CommandItem;
use crate::ui::ZediBoxWindow;
use crate::pybox;
use std::io::Write;
use std::{fs::File, io::BufReader};

pub fn create_commands() -> Vec<CommandItem> {
    let config_file = File::open("config.json").expect("Failed to open config file");
    let reader = BufReader::new(config_file);
    let commands: Vec<CommandItem> =
        serde_json::from_reader(reader).expect("Failed to parse config file");
    return commands;
}

pub fn save_commands(commands: Vec<CommandItem>) {
    // 将commands序列化成json字符串
    let json_str = serde_json::to_string(&commands).unwrap();

    // 将json字符串写入文件
    let mut config_file = File::create("config.json").unwrap();
    config_file.write_all(json_str.as_bytes()).unwrap();
}

pub fn run_command(command_item: CommandItem, zedi: &ZediBoxWindow) {
    let mut logs = zedi.get_logs();
    logs.push_str(&format!("\nrun command: {0}\n", command_item.name));
    let python_code: &str = command_item.code.as_str();
    let result = pybox::run_py_code(python_code);

    match result {
        Ok(msg) => {
            logs.push_str(msg);
        }
        Err(msg) => {
            logs.push_str(msg.as_str());
        }
    };

    logs.push_str("\n");
    zedi.set_logs(logs);
}
