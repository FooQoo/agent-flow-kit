import DefaultTheme from 'vitepress/theme'
import { inBrowser, type Theme } from 'vitepress'
import { rememberLanguageChoice } from './languageChoice'
import './custom.css'

export default {
  extends: DefaultTheme,
  enhanceApp({ siteData }) {
    if (inBrowser) rememberLanguageChoice(siteData.value.base)
  },
} satisfies Theme
