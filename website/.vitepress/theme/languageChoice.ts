export const languageStorageKey = 'agent-flow-kit-docs-lang'

export type DocsLang = 'en' | 'ja'

export function languageRedirectScript(base: string): string {
  const b = JSON.stringify(base)
  const key = JSON.stringify(languageStorageKey)
  return (
    `(function(){var b=${b},p=location.pathname;` +
    `if(p!==b&&p!==b+"index.html"&&p+"/"!==b)return;` +
    `var c=null;try{c=localStorage.getItem(${key})}catch(e){}` +
    `var l=c==="ja"||c==="en"?c:((navigator.languages&&navigator.languages[0])||navigator.language||"");` +
    `if(/^ja(-|$)/i.test(l))location.replace(b+"ja/"+location.search+location.hash)})()`
  )
}

function saveLanguage(lang: DocsLang) {
  try {
    localStorage.setItem(languageStorageKey, lang)
  } catch {
    // Ignore storage errors in restricted contexts.
  }
}

export function rememberLanguageChoice(base: string) {
  document.addEventListener(
    'click',
    (event) => {
      const target = event.target
      if (!(target instanceof Element)) return
      const link = target.closest<HTMLAnchorElement>(
        '.VPNavBarTranslations a[href], .VPNavScreenTranslations a[href]',
      )
      if (!link) return
      const path = new URL(link.href, location.href).pathname
      saveLanguage(path === `${base}ja` || path.startsWith(`${base}ja/`) ? 'ja' : 'en')
    },
    true,
  )
}
