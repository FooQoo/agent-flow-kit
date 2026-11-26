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
          { text: 'Guide', link: '/guide/getting-started' },
          { text: 'Design Philosophy', link: '/concepts/workflow-harness' },
          { text: 'Reference', link: '/reference/mcp' },
          { text: 'Troubleshooting', link: '/troubleshooting' },
        ],
        sidebar: [
          {
            text: 'Guide',
            items: [
              { text: 'Getting Started', link: '/guide/getting-started' },
              { text: 'Installation', link: '/guide/installation' },
              { text: 'Quickstart', link: '/guide/quickstart' },
            ],
          },
          {
            text: 'Design Philosophy',
            items: [
              { text: 'Workflow Harness', link: '/concepts/workflow-harness' },
              { text: 'Work Model', link: '/concepts/work-model' },
              { text: 'Human in / on the Loop', link: '/concepts/human-in-on-the-loop' },
              { text: 'Positioning', link: '/concepts/positioning' },
            ],
          },
          {
            text: 'Reference',
            items: [
              { text: 'MCP Reference', link: '/reference/mcp' },
            ],
          },
          {
            text: 'Support',
            items: [
              { text: 'Troubleshooting', link: '/troubleshooting' },
            ],
          },
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
          { text: 'ガイド', link: '/ja/guide/getting-started' },
          { text: '設計思想', link: '/ja/concepts/workflow-harness' },
          { text: 'リファレンス', link: '/ja/reference/mcp' },
          { text: 'トラブルシューティング', link: '/ja/troubleshooting' },
        ],
        sidebar: [
          {
            text: 'ガイド',
            items: [
              { text: 'Agent Flow Kit とは', link: '/ja/guide/getting-started' },
              { text: 'インストール', link: '/ja/guide/installation' },
              { text: '最初の仕事を任せる', link: '/ja/guide/quickstart' },
            ],
          },
          {
            text: '設計思想',
            items: [
              { text: '進行を支える仕組み', link: '/ja/concepts/workflow-harness' },
              { text: '仕事の進め方と 4 つの要素', link: '/ja/concepts/work-model' },
              { text: '人間と AI の役割分担', link: '/ja/concepts/human-in-on-the-loop' },
              { text: '既存の手法・ツールとの違い', link: '/ja/concepts/positioning' },
            ],
          },
          {
            text: 'リファレンス',
            items: [
              { text: 'MCP リファレンス', link: '/ja/reference/mcp' },
            ],
          },
          {
            text: 'サポート',
            items: [
              { text: 'トラブルシューティング', link: '/ja/troubleshooting' },
            ],
          },
        ],
        footer: {
          message: 'Agent Flow Kit — 仕事は AI に。主導権はあなたに。',
          copyright: 'Open Source · Anthropic および OpenAI の公式製品ではありません。',
        },
        outline: { label: 'このページの内容' },
        darkModeSwitchLabel: '外観',
        langMenuLabel: '言語を変更',
        skipToContentLabel: '本文へ移動',
        docFooter: { prev: '前のページ', next: '次のページ' },
      },
    },
  },
  themeConfig: {
    logo: { light: '/logo.png', dark: '/logo-dark.png', alt: 'Agent Flow Kit' },
    search: { provider: 'local' },
    socialLinks: [{ icon: 'github', link: 'https://github.com/FooQoo/agent-flow-kit' }],
  },
})
