with open('src/components/HistoryPanel.vue', 'rb') as f:
    data = f.read()

# Replace the strings in HistoryPanel.vue
data = data.replace(b'<button class="close-btn" @click="$emit(\'close\')">\xef\xbf\xbd/button>', b'<button class="close-btn" @click="$emit(\'close\')">\xe2\x95\xb3</button>')
data = data.replace(b'statusMsg.startsWith(\'\xef\xbf\xbd) }', b'statusMsg.startsWith(\'\xe2\x9d\x8c\') }')
data = data.replace(b'<button class="close-btn" @click="$emit(\'close\')">?/button>', b'<button class="close-btn" @click="$emit(\'close\')">\xe2\x95\xb3</button>')
data = data.replace(b'statusMsg.startsWith(\'?\') }', b'statusMsg.startsWith(\'\xe2\x9d\x8c\') }')

with open('src/components/HistoryPanel.vue', 'wb') as f:
    f.write(data)
