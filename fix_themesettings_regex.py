import sys
with open('src/components/ThemeSettings.vue', 'rb') as f:
    text = f.read().decode('utf-8', errors='ignore')

import re
old_slider = r'      <div class=\"setting-group\">\n        <label>.*?(?:px</label>|theme\.config\.marginLeft \}\}).*?<input.*?@input=\"theme\.setMargins\(.*?Number\(\(\$event\.target as HTMLInputElement\)\.value\).*?\).*?\n        />\n        <div class=\"range-labels\">\n          <span>.*?</span>\n          <span>.*?</span>\n        </div>\n      </div>'

new_sliders = '''      <div class="setting-group">
        <label>左边距：{{ theme.config.marginLeft }}px</label>
        <input
          type="range"
          :value="theme.config.marginLeft"
          @input="theme.setMargins(
            Number(($event.target as HTMLInputElement).value),
            theme.config.marginRight
          )"
          min="20"
          max="200"
          step="10"
        />
        <div class="range-labels">
          <span>窄</span>
          <span>宽</span>
        </div>
      </div>

      <div class="setting-group">
        <label>右边距：{{ theme.config.marginRight }}px</label>
        <input
          type="range"
          :value="theme.config.marginRight"
          @input="theme.setMargins(
            theme.config.marginLeft,
            Number(($event.target as HTMLInputElement).value)
          )"
          min="20"
          max="200"
          step="10"
        />
        <div class="range-labels">
          <span>窄</span>
          <span>宽</span>
        </div>
      </div>'''

text = re.sub(old_slider, new_sliders, text, flags=re.DOTALL)

with open('src/components/ThemeSettings.vue', 'w', encoding='utf-8') as f:
    f.write(text)
