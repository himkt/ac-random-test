## AC Random Test

CLI to run a program with randomly generated inputs (a.k.a. stress test), designed for programming contest (like AtCoder).

### Install

Use homebrew.

```
brew tap himkt/tap
brew install himkt/tap/ac-random-test
```

You can also download a binary from [GitHub Release](https://github.com/himkt/ac-random-test/releases).

### Basic usage

#### 1. Generating input randomly

`ac-random-test` assumes `gen.py` (executable) exists on the current directory.
`gen.py` should be executable like following:

##### `file gen.py`

```bash
gen.py: Python script text executable, ASCII text
```

> [!IMPORTANT]
> Make sure `gen.py` is executable.

##### `cat gen.py`

```python
#!/usr/bin/python3
import random
n = random.randint(1, 5)
a = [str(random.randint(0, 10)) for _ in range(n)]
with open("in.txt", "w") as f:
    print(n, file=f)
    print(" ".join(a), file=f)
```

> [!IMPORTANT]
> Don't forget to provide a shebang.

#### 2. Running a program

`ac-random-test` runs `./a.out` if you provide `a.out` and `./{argv0}` as `--run-cmd`.
`{argv0}` will be replaced with a name you give as a name (in this example, `a.out`).
`ac-random-test` also runs `./a.out_lazy` and check results are the same, in other words,
you need to prepare two programs: one is a submission program and the another is a
slow-but-correct program.

```bash
ac-random-test a.out --run-cmd './{argv0}'
```

If you want to run `ac-random-test` without a lazy program, specify `--without-lazy`.

```bash
ac-random-test a.out \
    --run-cmd './{argv0}' \
    --without-lazy
```

You may want to run a program written in other languages. No worries, please specify `--run-lazy-cmd`!

```bash
ac-random-test a.out \
    --run-cmd './{argv0}' \
    --run-lazy-cmd 'python3 slow_ac_code.py'
```

### 3. Setting time limit

In programing contest, your program must finish to run in a given time limit.
To find an input that makes your program slow, you can specify `--max-ms` to limit an execution time.
For example, if you provide `--max-ms 50`, `ac-random-test` fails if your program don't finished within 50ms.

```bash
ac-random-test a.out \
    --run-cmd './{argv0}' \
    --max-ms 50
```

Of course, `--max-ms` works with `--without-lazy`!

```bash
ac-random-test a.out \
    --run-cmd './{argv0}' \
    --max-ms 50 \
    --without-lazy
```

### 4. Using custom input generating scripts

`ac-random-test` assumes `gen.py` is created in the current directory by default.
You can use a custom script by specifying `--gen-input-cmd`.

```bash
ac-random-test a.out \
    --run-cmd './{argv0}' \
    --gen-input-cmd "python3 $HOME/Desktop/gen.py"
```
