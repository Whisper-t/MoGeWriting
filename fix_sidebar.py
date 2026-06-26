with open('src/components/Sidebar.vue', 'r', encoding='utf-8') as f:
    text = f.read()

text = text.replace('{{ collapsedVolumes.has(vol.id) ? \\\'▶\\\' : \\\'▼\\\' }}', '{{ collapsedVolumes.has(vol.id) ? \'▶\' : \'▼\' }}')

with open('src/components/Sidebar.vue', 'w', encoding='utf-8') as f:
    f.write(text)
