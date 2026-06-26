import os

with open('src/components/HistoryPanel.vue', 'r', encoding='utf-8') as f:
    text = f.read()

script_match = text.split('</script>')[0]
print(f"Single quotes: {script_match.count(chr(39))}")
print(f"Double quotes: {script_match.count(chr(34))}")
print(f"Backticks: {script_match.count(chr(96))}")
