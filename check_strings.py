import os

files = [
    'src/components/ExportDialog.vue',
    'src/components/HistoryPanel.vue',
    'src/components/TiptapEditor.vue',
]

with open('output.txt', 'w', encoding='utf-8') as out:
    for f in files:
        with open(f, 'r', encoding='utf-8') as fp:
            lines = fp.readlines()
        for i, line in enumerate(lines):
            if '?' in line:
                out.write(f'{f}:{i+1} - {line.strip()}\n')
