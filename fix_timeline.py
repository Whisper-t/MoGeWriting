with open('src/components/InspectorPanel.vue', 'r', encoding='utf-8') as f:
    text = f.read()

text = text.replace('title="删除此批?', 'title="删除此批注"')

with open('src/components/InspectorPanel.vue', 'w', encoding='utf-8') as f:
    f.write(text)

with open('src/components/TimelineView.vue', 'r', encoding='utf-8') as f:
    text2 = f.read()

text2 = text2.replace('{{ sortAsc ? \'? : \'? }}', '{{ sortAsc ? \'⬆\' : \'⬇\' }}')
text2 = text2.replace('<p>还没有随?/p>', '<p>还没有随笔</p>')

with open('src/components/TimelineView.vue', 'w', encoding='utf-8') as f:
    f.write(text2)
