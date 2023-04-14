def decorator1(func):
    def inner(*args, **kwargs):
        return func(*args, **kwargs)
    return inner

def route(path):
    def wrapper(handler):
        if path%2==0:
            return handler
        else:
            return sum_null
    return wrapper

def sum_null():
    return "nothing"

def decorator3(val):
    def decorator(func):
        return func
    return decorator

def decorator2(val):
    def decorator(func):
        def inner(*args, **kwargs):
            if val%2==0:
                return func(*args, **kwargs)
            else:
                return "value is not an even number"
        return inner
    return decorator

@decorator3(1)
def sum_even(a, b):
    return a+b

def sum_ev(a, b):
    return a+b


# print(decorator2(2)(sum_ev)(1,4))
# print(decorator3(1)(sum_ev(1,4)))
# print(sum_even(1,4))
# print((decorator3(1))(sum_ev(1,4)))
# print(sum_even(1,4))

def do(a):
    return a*a

my_list=[1,2,3,4,5]

# print(list(map(do, my_list)))
print(do.__name__)