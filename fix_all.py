import re
import os

def fix_file(filepath):
    with open(filepath, 'rb') as f:
        text = f.read().decode('utf-8', errors='ignore')
    
    # Common fixes
    # 1. Unclosed string literals caused by swallowed quotes `?"` or `?>`
    # We look for typical Chinese characters followed by `?` where a quote or newline should be.
    # Actually, we can just look for specific errors.
    
    if 'ExportDialog.vue' in filepath:
        text = re.sub(r'const bookName = computed\(\(\) => workspace\.bookIndex\?\.name \|\| \'未命\?.*?const bookType =', r'const bookName = computed(() => workspace.bookIndex?.name || \'未命名\')\nconst bookType =', text, flags=re.DOTALL)
        text = re.sub(r'exportStatus\.value = \'正在准备导出\?', r'exportStatus.value = \'正在准备导出...\'', text)
        text = re.sub(r'filters: \[\{ name: \'文本文件\?, extensions', r'filters: [{ name: \'文本文件\', extensions', text)
        text = re.sub(r'exportStatus\.value = \'✅ 导出成\?', r'exportStatus.value = \'✅ 导出成功\'', text)
        text = re.sub(r'exportStatus\.value = `❌ 导出失败：\?', r'exportStatus.value = `❌ 导出失败：${e}`', text)
        # Fix unclosed attributes in template
        text = re.sub(r'title=\"关闭\?>', r'title="关闭">\n', text)
        text = re.sub(r'title=\"选择导出目\?>', r'title="选择导出目录">\n', text)
        text = re.sub(r'title=\"点击选择\?>', r'title="点击选择">\n', text)
        text = re.sub(r'title=\"导出 \?', r'title="导出 ZIP">\n', text)
        text = re.sub(r'class=\"btn-primary\" @click=\"exportTxt\" :disabled=\"exporting\"\>.*?导出 TXT.*?\?', r'class="btn-primary" @click="exportTxt" :disabled="exporting">导出 TXT</button>', text, flags=re.DOTALL)
        text = re.sub(r'class=\"btn-primary\" @click=\"exportZip\" :disabled=\"exporting\"\>.*?导出 ZIP.*?\?', r'class="btn-primary" @click="exportZip" :disabled="exporting">导出 ZIP</button>', text, flags=re.DOTALL)
        text = re.sub(r'title=\"\?TXT 导出\">', r'title="纯文本导出">', text)
        text = re.sub(r'title=\"\?ZIP 备份\">', r'title="ZIP 备份">', text)
        text = re.sub(r'title=\"\?PDF 打印\">', r'title="PDF 打印">', text)
        # Any '?' that breaks syntax
        text = re.sub(r'\'未命\?', r'\'未命名\'', text)
        text = re.sub(r'\'随\?', r'\'随笔\'', text)

    elif 'HistoryPanel.vue' in filepath:
        text = re.sub(r'const loading = ref\(fal\?.*?const snapshots =', r'const loading = ref(false)\nconst snapshots =', text, flags=re.DOTALL)
        text = re.sub(r'title=\"关闭\?>', r'title="关闭">\n', text)
        text = re.sub(r'title=\"恢复此版\?>', r'title="恢复此版本">\n', text)
        text = re.sub(r'title=\"刷新历\?>', r'title="刷新历史">\n', text)

    elif 'ShortcutGuide.vue' in filepath:
        text = re.sub(r'title=\"关闭\?>', r'title="关闭">\n', text)
        text = re.sub(r'// \?键盘导\?const', r'// 键盘导航\nconst', text)
        text = re.sub(r'// 关闭弹\?const', r'// 关闭弹窗\nconst', text)

    elif 'Sidebar.vue' in filepath:
        text = re.sub(r'title=\"收起侧边\?>\?\s*</button>', r'title="收起侧边栏">\n        ◂\n      </button>', text)
        text = re.sub(r'title=\"展开侧边\?>\?\s*</button>', r'title="展开侧边栏">\n        ▸\n      </button>', text)

    elif 'TiptapEditor.vue' in filepath:
        text = re.sub(r'\'bold\?\s*:\s*editor\.isActive\(\'bold\'\)', r'\'bold\': editor.isActive(\'bold\')', text)
        text = re.sub(r'\'italic\?\s*:\s*editor\.isActive\(\'italic\'\)', r'\'italic\': editor.isActive(\'italic\')', text)
        text = re.sub(r'\'underline\?\s*:\s*editor\.isActive\(\'underline\'\)', r'\'underline\': editor.isActive(\'underline\')', text)
        text = re.sub(r'\'strike\?\s*:\s*editor\.isActive\(\'strike\'\)', r'\'strike\': editor.isActive(\'strike\')', text)
        text = re.sub(r'// 新建批\?const', r'// 新建批注\nconst', text)
        text = re.sub(r'// 选择的文本?const', r'// 选择的文本\nconst', text)
        # Any missing closing quote in attributes
        text = re.sub(r'title=\"加粗\?>', r'title="加粗">\n', text)
        text = re.sub(r'title=\"斜体\?>', r'title="斜体">\n', text)
        text = re.sub(r'title=\"下划\?>', r'title="下划线">\n', text)
        text = re.sub(r'title=\"删除\?>', r'title="删除线">\n', text)

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(text)

files = [
    'src/components/ExportDialog.vue',
    'src/components/HistoryPanel.vue',
    'src/components/ShortcutGuide.vue',
    'src/components/Sidebar.vue',
    'src/components/TiptapEditor.vue'
]

for f in files:
    try:
        fix_file(f)
        print(f"Fixed {f}")
    except Exception as e:
        print(f"Error fixing {f}: {e}")
