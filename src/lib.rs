pub mod command;
pub mod device;
pub mod ui;
pub mod pybox;

use pybox::init_pybox;
use slint::{ComponentHandle, Model};
use std::rc::Rc;
use ui::{CommandHandler, CommandItem, Device, DeviceHandler, ZediBoxWindow};
use uuid::Uuid;

pub struct State {
    pub zedi: ZediBoxWindow,
    pub commands: Rc<slint::VecModel<CommandItem>>,
    pub devices: Rc<slint::VecModel<Device>>,
}

pub fn init() -> State {
    let zedi = ZediBoxWindow::new().unwrap();
    let commands = Rc::new(slint::VecModel::from(command::create_commands()));
    let devices = Rc::new(slint::VecModel::from(device::create_devices()));

    zedi.global::<CommandHandler>().on_add({
        let weak_window = zedi.as_weak();
        let commands = commands.clone();
        move |mut command_item| {
            let window = weak_window.unwrap();
            // 随机长度16的字符串
            let uuid = Uuid::new_v4().to_string();
            command_item.uuid = uuid.into();
            commands.push(command_item);
            window.set_commands(commands.clone().into());
            let x = commands.iter().map(|item| item).collect();
            command::save_commands(x)
        }
    });

    zedi.global::<CommandHandler>().on_edit({
        let commands = commands.clone();
        move |command_item| {
            commands.iter().enumerate().for_each(|(index, item)| {
                if item.uuid == command_item.uuid {
                    commands.set_row_data(index, command_item.clone());
                }
            });
        }
    });

    zedi.global::<CommandHandler>().on_delete({
        let commands = commands.clone();
        move |command_item| {
            commands
                .iter()
                .enumerate()
                .for_each(|(index, item): (usize, CommandItem)| {
                    if item.uuid == command_item.uuid {
                        commands.remove(index);
                    }
                });
        }
    });

    zedi.on_load_devices({
        let ui_handle = zedi.as_weak();
        move || {
            if let Some(zedi) = ui_handle.upgrade() {
                let devices = device::create_devices();
                let devices_model = Rc::new(slint::VecModel::from(devices));
                zedi.set_devices(devices_model.into());
            } else {
                eprintln!("Failed to upgrade UI handle.");
            }
        }
    });

    zedi.on_load_commands({
        let ui_handle = zedi.as_weak();
        move || {
            if let Some(zedi) = ui_handle.upgrade() {
                let commands = command::create_commands();
                let commands_model = Rc::new(slint::VecModel::from(commands));
                zedi.set_commands(commands_model.into());
            } else {
                eprintln!("Failed to upgrade UI handle.");
            }
        }
    });

    // 实现全选调用
    zedi.global::<DeviceHandler>().on_select_all({
        let ui_handle = zedi.as_weak();
        move || {
            if let Some(zedi) = ui_handle.upgrade() {
                let devices: Vec<Device> = zedi
                    .get_devices()
                    .iter()
                    .map(move |mut item| {
                        item.selected = true;
                        item
                    })
                    .collect();
                zedi.set_devices(Rc::new(slint::VecModel::from(devices)).into());
            } else {
                eprintln!("Failed to upgrade UI handle.");
            }
        }
    });

    zedi.global::<DeviceHandler>().on_unselect_all({
        let ui_handle = zedi.as_weak();
        move || {
            if let Some(zedi) = ui_handle.upgrade() {
                let devices: Vec<Device> = zedi
                    .get_devices()
                    .iter()
                    .map(move |mut item| {
                        item.selected = false;
                        item
                    })
                    .collect();
                zedi.set_devices(Rc::new(slint::VecModel::from(devices)).into());
            } else {
                eprintln!("Failed to upgrade UI handle.");
            }
        }
    });

    zedi.global::<CommandHandler>().on_run({
        init_pybox(zedi.as_weak().unwrap());
        let ui_handle = zedi.as_weak();
        move |command_item| {
            let zedi = ui_handle.upgrade().unwrap();
            command::run_command(command_item, &zedi)
        }
    });

    State {
        zedi,
        commands,
        devices,
    }
}

pub fn main() {
    let state = init();

    state.zedi.invoke_load_commands();
    state.zedi.invoke_load_devices();
    state.zedi.run().unwrap();
}
