import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import es from './locales/es.json'
import fr from './locales/fr.json'
import de from './locales/de.json'
import zh from './locales/zh.json'
import ja from './locales/ja.json'
import pt from './locales/pt.json'
import ar from './locales/ar.json'
import hi from './locales/hi.json'
import ru from './locales/ru.json'

// Get initial locale from HTML lang attribute or default to 'en'
const getInitialLocale = () => {
  if (typeof document !== 'undefined') {
    return document.documentElement.lang?.split('-')[0] || 'en'
  }
  return 'en'
}

export const i18n = createI18n({
  legacy: false,
  locale: getInitialLocale(),
  fallbackLocale: 'en',
  messages: {
    en,
    es,
    fr,
    de,
    zh,
    ja,
    pt,
    ar,
    hi,
    ru,
  },
  globalInjection: true,
  silentFallbackWarn: true,
  silentTranslationWarn: true,
  missingWarn: false,
  fallbackWarn: false,
})

// Supported locales
export const supportedLocales = [
  { code: 'en', name: 'English', nativeName: 'English', rtl: false },
  { code: 'es', name: 'Spanish', nativeName: 'Español', rtl: false },
  { code: 'fr', name: 'French', nativeName: 'Français', rtl: false },
  { code: 'de', name: 'German', nativeName: 'Deutsch', rtl: false },
  { code: 'zh', name: 'Chinese', nativeName: '中文', rtl: false },
  { code: 'ja', name: 'Japanese', nativeName: '日本語', rtl: false },
  { code: 'pt', name: 'Portuguese', nativeName: 'Português', rtl: false },
  { code: 'ar', name: 'Arabic', nativeName: 'العربية', rtl: true },
  { code: 'hi', name: 'Hindi', nativeName: 'हिन्दी', rtl: false },
  { code: 'ru', name: 'Russian', nativeName: 'Русский', rtl: false },
]

// Locale utilities
export function setLocale(locale: string) {
  if (i18n.global.availableLocales.includes(locale)) {
    i18n.global.locale.value = locale
    if (typeof document !== 'undefined') {
      document.documentElement.lang = locale
      document.documentElement.dir = supportedLocales.find(l => l.code === locale)?.rtl ? 'rtl' : 'ltr'
    }
  }
}

export function getCurrentLocale() {
  return i18n.global.locale.value
}

export function getLocaleDirection(locale?: string) {
  const code = locale || getCurrentLocale()
  return supportedLocales.find(l => l.code === code)?.rtl ? 'rtl' : 'ltr'
}