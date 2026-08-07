#!/usr/bin/env python3
"""Extracts EVERY token from a source file. No filtering. No grouping. Just raw data."""

import re
import sys
from collections import defaultdict

SRC = "src/main.rs"
OUT_ORDERED = "analysis/all_tokens_ordered.txt"
OUT_FREQ = "analysis/token_frequencies.txt"

TOKEN_PATTERNS = [
    (r'r#"(?:[^"\\]|\\.)*"#', 'STRING_RAW'),
    (r'"(?:[^"\\]|\\.)*"', 'STRING'),
    (r"'(?:[^'\\]|\\.)'", 'CHAR'),
    (r'0[xX][0-9a-fA-F_]+\.?[0-9a-fA-F_]*[pP][+-]?\d+', 'NUM'),  # hex float
    (r'0[xX][0-9a-fA-F_]+', 'NUM'),
    (r'0[oO][0-7_]+', 'NUM'),
    (r'0[bB][01_]+', 'NUM'),
    (r'\d+\.\d+[eE][+-]?\d+', 'NUM'),
    (r'\d+[eE][+-]?\d+', 'NUM'),
    (r'\d+\.\d+', 'NUM'),
    (r'\d+', 'NUM'),
    (r'->', 'SYM'),
    (r'=>', 'SYM'),
    (r'::', 'SYM'),
    (r'\.\.=', 'SYM'),
    (r'\.\.', 'SYM'),
    (r'<<=', 'SYM'),
    (r'>>=', 'SYM'),
    (r'\+=', 'SYM'),
    (r'-=', 'SYM'),
    (r'\*=', 'SYM'),
    (r'/=', 'SYM'),
    (r'%=', 'SYM'),
    (r'\^=', 'SYM'),
    (r'&=', 'SYM'),
    (r'\|=', 'SYM'),
    (r'<<', 'SYM'),
    (r'>>', 'SYM'),
    (r'>=', 'SYM'),
    (r'<=', 'SYM'),
    (r'==', 'SYM'),
    (r'!=', 'SYM'),
    (r'&&', 'SYM'),
    (r'\|\|', 'SYM'),
    (r'//[^\n]*', 'SKIP'),
    (r'/\*[\s\S]*?\*/', 'SKIP'),
    (r'[ \t\r\n]+', 'SKIP'),
    (r'[a-zA-Z_][a-zA-Z0-9_]*', 'IDENT'),
    (r'.', 'SYM'),
]

tok_re = re.compile('|'.join(f'({p})' for p, _ in TOKEN_PATTERNS))
types = [t for _, t in TOKEN_PATTERNS]

freq = defaultdict(lambda: defaultdict(list))


def tokenize():
    lines = []
    with open(SRC, 'r') as f:
        for i, raw in enumerate(f, 1):
            line = raw.rstrip('\n')
            tokens = []
            pos = 0
            while pos < len(line):
                m = tok_re.match(line, pos)
                if not m:
                    pos += 1
                    continue
                for grp_idx in range(len(types)):
                    val = m.group(grp_idx + 1)
                    if val is not None:
                        if types[grp_idx] == 'SKIP':
                            break
                        tokens.append(val)
                        freq[val][i].append(i)
                        break
                pos = m.end()
            lines.append((i, tokens))
    return lines


def fmt_lines(entries):
    out = []
    for line_num, tokens in entries:
        out.append(f"L{line_num:04d}: {' | '.join(tokens)}")
    return '\n'.join(out) + '\n'


def fmt_freq(freq):
    out = []
    for token in sorted(freq.keys(), key=lambda t: (t.upper(), t)):
        linemap = freq[token]
        total = sum(len(v) for v in linemap.values())
        lines_unique = sorted(linemap.keys())
        lines_str = ', '.join(f'L{l}' for l in lines_unique)
        escaped = token.replace('\\', '\\\\').replace('"', '\\"')
        out.append(f'"{escaped}" {total} [{lines_str}]')
    return '\n'.join(out) + '\n'


def main():
    entries = tokenize()
    with open(OUT_ORDERED, 'w') as f:
        f.write(fmt_lines(entries))
    with open(OUT_FREQ, 'w') as f:
        f.write(fmt_freq(freq))
    total = sum(len(tokens) for _, tokens in entries)
    uniq = len(freq)
    print(f"Extracted {total} tokens ({uniq} unique) from {SRC}")
    print(f"  -> {OUT_ORDERED}")
    print(f"  -> {OUT_FREQ}")


if __name__ == '__main__':
    main()
