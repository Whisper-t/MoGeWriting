import re

with open('src/components/Sidebar.vue', 'r', encoding='utf-8', errors='ignore') as f:
    text = f.read()

text = re.sub(r'title="在此卷添加章\?', 'title="在此卷添加章节"\n', text)
text = re.sub(r'title="重命\?', 'title="重命名"\n', text)
text = re.sub(r'title="删除\?', 'title="删除"\n', text)
text = re.sub(r'title="新建\?', 'title="新建"\n', text)
text = re.sub(r'title="添加章\?', 'title="添加章节"\n', text)
text = re.sub(r'title="编辑书籍信\?', 'title="编辑书籍信息"\n', text)

with open('src/components/Sidebar.vue', 'w', encoding='utf-8') as f:
    f.write(text)
