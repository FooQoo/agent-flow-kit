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
      description: 'A lightweight harness for AI agents to carry out work.',
      themeConfig: {
        nav: [
          { text: 'Install', link: '/guide/' },
          { text: 'Quickstart', link: '/guide/quickstart' },
        ],
        footer: {
          message: 'Agent Flow Kit — Delegate work. Stay in control.',
          copyright: 'Open Source · Not affiliated with Anthropic or OpenAI.',
        },
      },
    },
    ja: {
      label: '日本語',
      lang: 'ja',
      link: '/ja/',
      description: 'AI の仕事の遂行を支える軽量なハーネス。',
      themeConfig: {
        nav: [
          { text: 'インストール', link: '/ja/guide/' },
          { text: 'クイックスタート', link: '/ja/guide/quickstart' },
        ],
        footer: {
          message: 'Agent Flow Kit — Delegate work. Stay in control.',
          copyright: 'Open Source · Anthropic および OpenAI の公式製品ではありません。',
        },
        outline: { label: 'このページの内容' },
        darkModeSwitchLabel: '外観',
        langMenuLabel: '言語を変更',
        skipToContentLabel: '本文へ移動',
      },
    },
  },
  themeConfig: {
    logo: { light: '/logo.png', dark: '/logo-dark.png', alt: 'Agent Flow Kit' },
    socialLinks: [{ icon: 'github', link: 'https://github.com/FooQoo/agent-flow-kit' }],
  },
})
