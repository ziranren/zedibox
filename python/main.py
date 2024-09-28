import string
import random
import time
import slint
import json

from slint import Color, ListModel, Timer, TimerMode
from pybox import run_command
# from slint.loader.app import CommandItem

style = None # "fluent-light"
zedibox = slint.load_file("ui/zedibox.slint", style=style)

class ZediboxApp(zedibox.ZediBoxWindow):
    def __init__(self):
        super().__init__()
        self.load_commands()
        self.load_devices()
    
    @slint.callback
    def load_devices(self):
        # 随机长度为16 的字符串
        sn = ''.join(random.choices(string.ascii_letters + string.digits, k=16))

        self.devices = ListModel([
            zedibox.Device(**{ 'selected': True, 'port': 0, 'sn': sn, 'state': "OFF", 'signal': True, 'version': "Version 10.12.121.0" }),
            zedibox.Device(**{ 'selected': False, 'port': 1, 'sn': sn, 'state': "ON", 'signal': False, 'version': "Version 10.12.121.0" }),
            zedibox.Device(**{ 'selected': False, 'port': 2, 'sn': sn, 'state': "ON", 'signal': True, 'version': "Version 10.12.121.0" }),
            zedibox.Device(**{ 'selected': False, 'port': 3, 'sn': sn, 'state': "ON", 'signal': True, 'version': "Version 10.12.121.0" }),
            zedibox.Device(**{ 'selected': True, 'port': 4, 'sn': sn, 'state': "ON", 'signal': True, 'version': "Version 10.12.121.0" }),
            zedibox.Device(**{ 'selected': False, 'port': 5, 'sn': sn, 'state': "ON", 'signal': True, 'version': "Version 10.12.121.0" }),
            zedibox.Device(**{ 'selected': False, 'port': 6, 'sn': sn, 'state': "ON", 'signal': True, 'version': "Version 10.12.121.0" }),
            zedibox.Device(**{ 'selected': False, 'port': 7, 'sn': sn, 'state': "ON", 'signal': True, 'version': "Version 10.12.121.0" })
        ])
    
    @slint.callback
    def load_commands(self):
        commands = json.load(open("config.json", "r"))
        commands = [zedibox.CommandItem(**command) for command in commands]
        self.commands = ListModel(commands)
    
    @slint.callback(global_name="CommandHandler", name="run")
    def on_command_clicked(self, item):
        run_command(item, self)
    
    @slint.callback(global_name="CommandHandler", name="add")
    def on_command_add(self, item):
        item.uuid = ''.join(random.choices(string.ascii_letters + string.digits, k=16))
        self.commands.append(item)
        self.save_config()
    
    @slint.callback(global_name="CommandHandler", name="edit")
    def on_command_edit(self, item):
        for i in range(len(self.commands)):
            command = self.commands[i]
            if command.uuid == item.uuid:
                self.commands[i] = item
                break
        self.save_config()

    @slint.callback(global_name="CommandHandler", name="delete")
    def on_command_delete(self, item):
        for i in range(len(self.commands)):
            command = self.commands[i]
            if command.uuid == item.uuid:
                del self.commands[i]
                break
        self.save_config()

    # @slint.callback(global_name="CommandHandler", name="dialog")
    # def dialog(self, item):
    #     print('show dialog')
    #     self.show_command_dialog(item)
    
    @slint.callback(global_name="DeviceHandler", name="select_all")
    def select_all(self):
        for i in range(len(self.devices)):
            device = self.devices[i]
            device.selected = True
            self.devices[i] = device
    
    @slint.callback(global_name="DeviceHandler", name="unselect_all")
    def unselect_all(self):
        for i in range(len(self.devices)):
            device = self.devices[i]
            device.selected = False
            self.devices[i] = device

    def log_info(self, msg):
        msg = str(msg)
        self.logs = self.logs + msg + "\n"
    
    def get_all_selected_device(self):
        selected = []
        for device in self.devices:
            if device.selected:
                selected.append(device)
        return selected

    def save_config(self):
        commands = [dict(i) for i in list(self.commands)]
        json.dump(commands, open("config.json", "w"))

if __name__ == "__main__":
    app = ZediboxApp()
    app.run()
