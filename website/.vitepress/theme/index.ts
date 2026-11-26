import DefaultTheme from 'vitepress/theme'
import { inBrowser, type Theme } from 'vitepress'
import LandingPage from './LandingPage.vue'
import { rememberLanguageChoice } from './languageChoice'
import './custom.css'

export default {
  extends: DefaultTheme,
  enhanceApp({ app, siteData }) {
    app.component('LandingPage', LandingPage)
    if (inBrowser) rememberLanguageChoice(siteData.value.base)
  },
} satisfies Theme
