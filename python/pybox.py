from functools import partial

class PyBox:

    def __init__(self, app):
        self.app = app
    def selected(self):
        return self.app.get_all_selected_device()

    def log_info(self, msg):
        return self.app.log_info(msg)
def run_command(command, app=None):
    code = command.code
    if not code:
        print(f"{command.name}, No code to run")
        return
    globals = {
        "__name__": "__main__",
        "__file__": "__main__",
        "__builtins__": __builtins__,
        "zedi": PyBox(app),
        "print": partial(print, "pybox:"),
    }
    exec(code, globals)
