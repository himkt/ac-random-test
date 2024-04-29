
#### Generating input randomly

`ac-random-test` assumes `gen.py` (executable) exists on the current directory.

```sh
> file gen.py
gen.py: Python script text executable, ASCII text
```

```python
> cat gen.py
#!/usr/bin/python3
import random
n = random.randint(1, 5)
a = [str(random.randint(0, 10)) for _ in range(n)]
with open("in.txt", "w") as f:
    print(n, file=f)
    print(" ".join(a), file=f)
```
