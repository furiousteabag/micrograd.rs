```
a = 2
b = -3
c = 10
d = -2
e = a \* b = -6
g = e + c = 4
l = g \* d = -8

df/dx = (f(x + h) - f(x)) / h, h -> 0

1. dl / dl = 1
   f(l) = l
   (f(l + h) - f(l)) / h = (l + h - l) / h = h / h = 1
2. dl / dd = g = 4
   f(d) = g \* d
   (f(d + h) - f(d)) / h = (g \* (d + h) - g \* d) / h = (g \* d + g \* h - g \* d) / h = g \* h / h = g
3. dl / dg = d
4. dl / de = (dl / dg) \* (dg / de) = d \* 1 = d
   dg / de
   f(e) = e + c
   (f(e + h) - f(e)) / h = (e + h + c - e - c) / h = h / h = 1
5. dl / dc = (dl / dg) \* (dg / dc) = d \* 1 = d
6. dl / da = (dl / dg) \* (dg / de) \* (de \ da) = d \* 1 \* b = d \* b
   de \ da = b
7. dl / db = (dl / dg) \* (dg / de) \* (de \ db) = d \* 1 \* a = d \* a
```
