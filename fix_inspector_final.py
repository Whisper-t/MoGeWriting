import re
with open('src/components/InspectorPanel.vue', 'r', encoding='utf-8', errors='ignore') as f:
    text = f.read()

text = re.sub(r'title="删除此批\?', 'title="删除此批注"\n', text)

with open('src/components/InspectorPanel.vue', 'w', encoding='utf-8') as f:
    f.write(text)
