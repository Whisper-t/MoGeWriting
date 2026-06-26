import os

def fix(filepath):
    with open(filepath, 'rb') as f:
        text = f.read().decode('utf-8', errors='ignore')

    text = text.replace('<button class="close-btn" @click="$emit(\'close\')">?/button>', '<button class="close-btn" @click="$emit(\'close\')">✕</button>')
    text = text.replace('{{ restoring ? \'恢复? : \'🔄 恢复此版? }}', '{{ restoring ? \'恢复中...\' : \'🔄 恢复此版本\' }}')
    text = text.replace('statusMsg.startsWith(\'?\') }', 'statusMsg.startsWith(\'❌\') }')
    
    text = text.replace('// 监听外部内容变化（切换文件时?watch(', '// 监听外部内容变化（切换文件时）\nwatch(')

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(text)

fix('src/components/HistoryPanel.vue')
fix('src/components/TiptapEditor.vue')
