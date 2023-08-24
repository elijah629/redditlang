hljs.registerLanguage("redditlang", (hljs) => ({
  name: "RedditLang",
  keywords: {
    $pattern: /[\w']+/, 
    keyword: "is but isn't spez repeatdatshid test wall sthu call shoot weneed bringme school damn meth callmeonmycellphone",
    literal: "wat Yup Nope Dunno Huh Yeet",
    built_in: "zzz coitusinterruptus pulloutnt exit nums"
  },
  contains: [
    hljs.QUOTE_STRING_MODE,
    hljs.C_NUMBER_MODE,
    {
      scope: "string",
      begin: '"',
      end: '"',
      contains: [{ begin: "\\\\." }],
    },
    hljs.COMMENT(/#\*/, /\*#/),
    hljs.COMMENT(/#/, /$/),
  ],
}));

hljs.initHighlightingOnLoad();
