import { defineConfig } from 'vitepress'
import { languageRedirectScript } from './theme/languageChoice'

const base = process.env.DOCS_BASE || '/'

export default defineConfig({
  title: 'Agent Flow Kit',
  base,
  cleanUrls: true,
  head: [['link', { rel: 'icon', type: 'image/png', href: `${base}logo.png` }]],
  transformHtml(html, _id, { pageData }) {
    if (pageData.relativePath !== 'index.md') return
    const charset = '<meta charset="utf-8">'
    return html.replace(charset, `${charset}\n    <script>${languageRedirectScript(base)}</script>`)
  },
  locales: {
    root: {
      label: 'English',
      lang: 'en',
      description: 'Plan, track, and resume work with your coding agent.',
      themeConfig: {
        nav: [
          { text: 'Install', link: '/guide/' },
          { text: 'Quickstart', link: '/guide/quickstart' },
        ],
      },
    },
    ja: {
      label: '日本語',
      lang: 'ja',
      link: '/ja/',
      description: 'AI エージェントに任せた作業の計画と進捗を、プロジェクトに残す。',
      themeConfig: {
        nav: [
          { text: 'インストール', link: '/ja/guide/' },
          { text: 'クイックスタート', link: '/ja/guide/quickstart' },
        ],
        outline: { label: 'このページの内容' },
        darkModeSwitchLabel: '外観',
        langMenuLabel: '言語を変更',
        skipToContentLabel: '本文へ移動',
      },
    },
  },
  themeConfig: {
    logo: { light: '/logo.png', dark: '/logo-dark.png', alt: 'Agent Flow Kit' },
  },
})
