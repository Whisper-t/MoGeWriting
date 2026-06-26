import os

def check(f):
    with open(f, 'r', encoding='utf-8') as fp:
        lines = fp.readlines()
    for i, line in enumerate(lines):
        if '?' in line:
            print(f"{f}:{i+1} - {line.strip()}")

check('src/components/HistoryPanel.vue')
check('src/components/TiptapEditor.vue')
