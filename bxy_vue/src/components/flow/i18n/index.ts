import translations from './language-zh'

/**
 * 自定义翻译
 * @param template
 * @param replacements
 * @returns
 */
export default function customTranslate(template: string, replacements: Record<string, string>) {
  replacements = replacements || {}

  // Translate
  template = translations[template as keyof typeof translations] || template

  // Replace
  return template.replace(/{([^}]+)}/g, function (_, key) {
    return replacements[key] || '{' + key + '}'
  })
}
