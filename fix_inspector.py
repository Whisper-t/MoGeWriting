import re
with open('src/components/InspectorPanel.vue', 'r', encoding='utf-8', errors='ignore') as f:
    text = f.read()

text = re.sub(r'title="删除\?', 'title="删除"\n', text)
text = re.sub(r'title="编辑\?', 'title="编辑"\n', text)
text = re.sub(r'title="新建\?', 'title="新建"\n', text)
text = re.sub(r'title="关闭\?', 'title="关闭"\n', text)
text = re.sub(r'title="保存\?', 'title="保存"\n', text)
text = re.sub(r'title="添加批\?', 'title="添加批注"\n', text)
text = re.sub(r'placeholder="添加新灵\?', 'placeholder="添加新灵感"\n', text)

with open('src/components/InspectorPanel.vue', 'w', encoding='utf-8') as f:
    f.write(text)

with open('src/components/TimelineView.vue', 'r', encoding='utf-8', errors='ignore') as f:
    text2 = f.read()

text2 = re.sub(r'title="返回上一\?', 'title="返回上一页"\n', text2)
with open('src/components/TimelineView.vue', 'w', encoding='utf-8') as f:
    f.write(text2)
