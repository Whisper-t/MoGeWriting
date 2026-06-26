import os

def rep(filepath, search, replace):
    with open(filepath, 'rb') as f:
        text = f.read().decode('utf-8', errors='ignore')
    text = text.replace(search, replace)
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(text)

rep('src/components/ExportDialog.vue', 'exportStatus.value = \'?备份已恢复成功！请重新打开当前作品\'', 'exportStatus.value = \'✅ 备份已恢复成功！请重新打开当前作品\'')
rep('src/components/ExportDialog.vue', 'exportStatus.value = `?恢复失败?{e}`', 'exportStatus.value = `❌ 恢复失败：${e}`')
rep('src/components/ExportDialog.vue', 'exportStatus.startsWith(\'?\')', 'exportStatus.startsWith(\'❌\')')

rep('src/components/HistoryPanel.vue', 'statusMsg.value = \'?已恢复到该版?', 'statusMsg.value = \'✅ 已恢复到该版本\'')
rep('src/components/HistoryPanel.vue', 'statusMsg.value = `?恢复失败?{e}`', 'statusMsg.value = `❌ 恢复失败：${e}`')
rep('src/components/HistoryPanel.vue', 'statusMsg.startsWith(\'?\')', 'statusMsg.startsWith(\'❌\')')

rep('src/components/ShortcutGuide.vue', 'title=\"使用说\?>', 'title=\"使用说明\">\n')

rep('src/components/TiptapEditor.vue', 'title=\"清空排\?>', 'title=\"清空排版\">\n')
rep('src/components/TiptapEditor.vue', 'title=\"撤销 \?>', 'title=\"撤销\">\n')
rep('src/components/TiptapEditor.vue', 'title=\"重做 \?>', 'title=\"重做\">\n')
