from dataclasses import dataclass

@dataclass
class Prova:
    a: int 
    b: int = 2
    c: bool = True

    @property 
    def get_sum(self):
        return self.b + self.a

prova = Prova(3)
print(prova.b)
p = Prova(1, c = False)
print(p.c)
print(p.get_sum)

def fun(dict): 
    dict["a"] += 1

dict1 = {"a": 0}
fun(dict1)
print(dict1["a"])

list1 = [1,2,3]
while len(list1) != 0: 
    print("again")
    list1.remove(list1[0])
print(list1)

tup = (1, 2)
if 1 in tup: print("yes")

class St:
    def __init__(self, cost):
        self.cost = cost

st1 = St(3)
print("ST1", st1.cost)
st2 = St(2)
lst = []
lst.append(st1)
lst.append(st2)
lst.sort(key=lambda x: x.cost)
print("lst", lst[0].cost)
print("cost first", lst[0].cost)


id = 1
def get_id():
    global id
    id += 1
    return id

print(get_id())

print([-1] * 3)

lst = [1,2,3]
dictio = {}
dictio['a'] = (1,2)
print(dictio['a'])