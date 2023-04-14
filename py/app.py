import python.api as api


app= api.API()


@app.route("home")
def home(context, key, val):
        print("home")
        key= key + "1"
        context[key]=val

@app.route("about")
def about(context, key, val):
        print("about")
        key= key + "1"
        context[key]=val

print(app.__dict__)
print(app({"path":"hello"}, "new_key", "new_value"))
print(app.__dict__)

# context={"path":"home"}
# home(context, "new_key", "new_value")
# print(context)

