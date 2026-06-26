with open('src/components/InspectorPanel.vue', 'rb') as f:
    data = f.read()

# Replace any title="删除此批.*?" with title="删除此批注"
import re
data = re.sub(b'title="\xe5\x88\xa0\xe9\x99\xa4\xe6\xad\xa4\xe6\x89\xb9.*?\n', b'title="\xe5\x88\xa0\xe9\x99\xa4\xe6\xad\xa4\xe6\x89\xb9\xe6\xb3\xa8"\n', data)

with open('src/components/InspectorPanel.vue', 'wb') as f:
    f.write(data)
