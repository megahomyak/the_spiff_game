from types import SimpleNamespace as SN
import os

def allfiles(dir):
    for root, _, files in os.walk(dir):
        for file in files:
            yield os.path.join(root, file)

from types import SimpleNamespace as SN

def parse(inp):
    def parsell(unc, unx, inp):
        stack = [(0, [])]
        for i, c in enumerate(inp):
            if c == "(":
                new = (i, [])
                stack[-1][1].append(new)
                stack.append(new)
            elif c == ")":
                if len(stack) == 1:
                    unx.append(i)
                else:
                    stack.pop()
            else:
                stack[-1][1].append((i, c))
        while len(stack) != 1:
            unc.append(stack.pop()[0])
        return stack[0][1]
    def prettify(parsed_inp):
        sbuf = ""
        sbufidx = 0
        pretty = []
        for idx, layer in parsed_inp:
            if isinstance(layer, str):
                if not sbuf:
                    sbufidx = idx
                sbuf += layer
            else:
                if sbuf:
                    pretty.append((sbufidx, sbuf))
                    sbuf = ""
                pretty.append((idx, prettify(layer)))
        if sbuf:
            pretty.append((sbufidx, sbuf))
        return pretty
    def name(prettified):
        named = []
        for idx, layer in prettified:
            named.append(SN(idx=idx, value=layer if isinstance(layer, str) else name(layer)))
        return named
    unx = []
    unc = []
    res = name(prettify(parsell(unx, unc, inp)))
    return SN(unexpected_closers=unx, unclosed_openers=unc, root=res)

def stringify(node):
    tasks = [node.value]
    result = ""
    while tasks:
        task = tasks.pop()
        if isinstance(task, str):
            result += task
        elif isinstance(task, list):
            tasks.append(")")
            for subnode in task[::-1]:
                tasks.append(subnode.value)
            tasks.append("(")
    return result

def pairs(node):
    for i in range(len(node.value) // 2):
        try:
            yield node.value[i*2].value.strip(), node.value[i*2+1]
        except AttributeError:
            raise Exception(node.value[i*2].idx)

def attrs(node):
    return dict(pairs(node))

def main():
    assets = set(allfiles("assets"))
    parsed = parse(open("game.und", encoding="utf-8").read())
    assert parsed.unclosed_openers == []
    assert parsed.unexpected_closers == []
    parsed = attrs(parsed.root)
    assert len(assets) == 0

main()
