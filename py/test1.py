import ast
import functools
import os
from pathlib import Path

class X:
    def __init__(self, a):
        self.a= a
# path= os.getcwd()
# my_path= os.path.join(path, "tes.py")
# print(my_path)
# print(os.path.exists(my_path))
# pa= Path(__file__).resolve(strict=True).parent /"app.py"

# print(os.getcwd())

def hello():
    print("hello")

# node= ast.parse(open("./test.py").read())
# print(node.body)

# class Visitor(ast.NodeVisitor):
#     def visit_Str(self, node):
#         print('String Node: "' + node.s + '"')

# class MyTransformer(ast.NodeTransformer):
#     def visit_Str(self, node):
#         return ast.Str('str: '+node.s)
        
# parsed= ast.parse("print('Hello World')")
# MyTransformer().visit(parsed)
# Visitor().visit(parsed)

class Visitor(ast.NodeVisitor):
    list_=[]
    def visit_FunctionDef(self, node):
        print("function", node.name, node.lineno, node.end_col_offset)

    def visit_ClassDef(self, node):
        self.list_.append(node.name.lower())
        print("class", node.name.lower, node.lineno, node.end_col_offset)

    def visit(self, node):
        super().visit(node)
        return self.list_

# class MyTransformer(ast.NodeTransformer):
#     def visit_Str(self, node):
#         return ast.Str(node.s)
        
parsed= ast.parse(open("./test1.py").read())
# MyTransformer().visit(parsed)
b= Visitor().visit(parsed)
print(b)
# Visitor().visit(open("./test.py").read())


def hi(name, house):
    print(name, house)

d= {'name': "mubby", 'hous': "erin"}

hi(**d)