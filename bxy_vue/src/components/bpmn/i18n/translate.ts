import i18nTranslation_zh from './language-zh'

/**
 * 自定义翻译
 * @param template
 * @param replacements
 * @returns
 */
export default function translate(template: string, replacements: Record<string, string>) {
  replacements = replacements || {}

  // Translate
  template = i18nTranslation_zh[template as keyof typeof i18nTranslation_zh] || template

  // Replace
  return template.replace(/{([^}]+)}/g, function (_, key) {
    return replacements[key] || '{' + key + '}'
  })
}
