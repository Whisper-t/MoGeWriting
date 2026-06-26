with open('src/components/ThemeSettings.vue', 'r', encoding='utf-8') as f:
    text = f.read()

import re
old_slider = r'''      <div class="setting-group">
        <label>左右边距：{{ theme.config.marginLeft }}px</label>
        <input
          type="range"
          :value="theme.config.marginLeft"
          @input="theme.setMargins(
            Number\(\(\$event.target as HTMLInputElement\).value\),
            Number\(\(\$event.target as HTMLInputElement\).value\)
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

text = re.sub(old_slider, new_sliders, text)

with open('src/components/ThemeSettings.vue', 'w', encoding='utf-8') as f:
    f.write(text)
