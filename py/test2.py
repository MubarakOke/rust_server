# import inspect
# class A:
#     pass

# class B(A):
#     pass

# class C(B):
#     @staticmethod
#     def hi():
#         print("saying hi")

# def hello(name, **kwargs):
#     print(f"{kwargs['greating']} to {name}")

# print(inspect.signature(hello))
# d=inspect.signature(hello)
# greet={'greating':"hello"}
# name= "mubarak"
# print(d.bind(name, **greet))
# # d= inspect.getmro(C)[0]
# # d.hi()

c= {1:"hello", 2:"hi"}
d= {3:"three"}
print(d.get(**{3:"three"}))