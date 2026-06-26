with open('src/components/HistoryPanel.vue', 'r', encoding='utf-8') as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if '加载失败' in line:
        lines[i] = '    previewContent.value = `❌ 加载失败：${e}`\n'
    elif '恢复失败' in line:
        lines[i] = '    statusMsg.value = `❌ 恢复失败：${e}`\n'
    elif '已恢复到该版' in line:
        lines[i] = '    statusMsg.value = \'✅ 已恢复到该版本\'\n'

with open('src/components/HistoryPanel.vue', 'w', encoding='utf-8') as f:
    f.writelines(lines)
