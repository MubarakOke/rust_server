class API:
    def __init__(self):
        self.routes= {}
        self.context={}

    def __call__(self, val, new_key, new_value):
        path= val.get("path", None)
        handler= self.routes.get(path, None)
        if handler:
            handler(self.context, new_key, new_value)    
            return path
        else:
            return f"no route for {path}"

    def route(self, path):
        def inner(func):
            self.routes[path] = func
            return func
        return inner