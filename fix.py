import re
import os

with open('src/views/HomeView.vue', 'rb') as f:
    text = f.read().decode('utf-8', errors='ignore')

# Fix const declarations commented out
text = re.sub(r'// 新建对话\?const', r'// 新建对话框\nconst', text)
text = re.sub(r'// 恢复状\?const', r'// 恢复状态\nconst', text)

# Fix title
text = re.sub(r'title=\"快捷键设\?>', r'title=\"快捷键设置\">\n', text)
text = re.sub(r'\?>\s*</button>', r'⌨\n          </button>', text)

# Fix theme icons
text = re.sub(r'theme.config.mode === \'dark\' \? \'\? : \'\? \}\}', r'theme.config.mode === \'dark\' ? \'🌙\' : \'☕\' }}', text)
text = re.sub(r'\'dark\' \? \'\? : \'\?', r'\'dark\' ? \'🌙\' : \'☕\'', text)

with open('src/views/HomeView.vue', 'w', encoding='utf-8') as f:
    f.write(text)
