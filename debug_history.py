import os

with open('src/components/HistoryPanel.vue', 'r', encoding='utf-8') as f:
    text = f.read()

script_match = text.split('</script>')[0]
lines = script_match.split('\n')

open_count = 0
close_count = 0
for i, line in enumerate(lines):
    open_count += line.count('{')
    close_count += line.count('}')
    if line.count('{') != line.count('}'):
        print(f'{i+1}: {line.strip()} (open: {line.count("{")}, close: {line.count("}")})')

print(f"Total open: {open_count}, Total close: {close_count}")
