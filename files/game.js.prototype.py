FINDING_NAME = 0
READING_NAME = 1
READING_INPUT = 2

def exec_(prog):
    stage = FINDING_NAME
    namebuf = ""
    inputbuf = ""
    nesting = 0
    for c in prog:
        if stage == FINDING_NAME:
            if c == ".":
                stage = READING_NAME
        elif stage == READING_NAME:
            if c == "(":
                stage = READING_INPUT
            else:
                namebuf += c
        elif stage == READING_INPUT:
            if c == ")" and nesting == 0:
                invoke_fn(namebuf, inputbuf)
                namebuf = ""
                inputbuf = ""
            else:
                inputbuf += c
                if c == ")":
                    nesting -= 1
                elif c == "(":
                    nesting += 1

def invoke_fn(fnname, input_):
    if fnname == "exec":
        exec_(input_)
    elif fnname == "bg"
