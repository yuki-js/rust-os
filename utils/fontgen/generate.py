#!/usr/bin/env python3

textfile = open("hankaku.txt", "r")
out = ""
moji = ""
for line in textfile:
    if line[0:4] == "char":
        moji = ""
        continue
    if line == "\n":
        out+="b\"%s\",\n" % moji
        continue
    line = line.replace(".", "0").replace("*", "1")
    moji+="\\x%02x" % int(line, 2)
print(out)
