r"""Scrubs machine paths from a transcript on stdin.

Replaces C:\Users\<name> with ~, and the repository root with <repo>, in each of
the spellings a Windows toolchain under Git Bash produces: /d/repos/x (the
shell), D:\repos\x (rustc and cargo) and D:/repos/x. The root is argv[1], in
the shell's spelling. No regex, because sed dialects disagree on backslashes and
the first run of this script left home paths in two transcripts that way.
"""
import sys

BS = chr(92)


def root_spellings(root):
    """The repository root as the shell, rustc and cargo each print it.

    Git Bash may already have converted argv[1] from /d/rest to D:/rest before
    Python sees it, so both are accepted.
    """
    root = root.replace(BS, "/").rstrip("/")
    parts = root.split("/")
    if len(parts) > 2 and parts[0] == "" and len(parts[1]) == 1:
        drive, rest = parts[1], parts[2:]
    elif len(parts) > 1 and len(parts[0]) == 2 and parts[0][1] == ":":
        drive, rest = parts[0][0], parts[1:]
    else:
        return [root]
    spellings = []
    for d in (drive.lower(), drive.upper()):
        spellings.append("/" + d + "/" + "/".join(rest))
        spellings.append(d + ":" + BS + BS.join(rest))
        spellings.append(d + ":/" + "/".join(rest))
    # Longest first, so no spelling is left half-replaced by a shorter one.
    return sorted(set(spellings), key=len, reverse=True)


def scrub_home(data):
    out = []
    i = 0
    needle = "C:" + BS + "Users" + BS
    while True:
        j = data.find(needle, i)
        if j < 0:
            out.append(data[i:])
            break
        out.append(data[i:j])
        out.append("~")
        k = data.find(BS, j + len(needle))
        i = k if k >= 0 else len(data)
    return "".join(out)


data = sys.stdin.read()
if len(sys.argv) > 1 and sys.argv[1]:
    for spelling in root_spellings(sys.argv[1]):
        data = data.replace(spelling, "<repo>")
sys.stdout.write(scrub_home(data))
