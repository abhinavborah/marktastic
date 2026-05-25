// Thesis Chapter Template
// Formal thesis chapter with numbered sections

#set page(
  paper: "a4",
  margin: (top: 2.8cm, bottom: 2.8cm, left: 3.5cm, right: 2.5cm),
  numbering: "1",
  header: context [
    #if counter(page).get().first() > 1 {
      align(right, text(size: 9pt, fill: gray)[Chapter])
    }
  ],
  footer: align(center, context {
    if counter(page).get().first() > 1 {
      counter(page).display("i")
    }
  }),
)

#set par(
  first-line-indent: 1.5em,
  justify: true,
)

#set text(
  font: "Libertinus Serif",
  size: 12pt,
  lang: "en",
)

#set heading(
  numbering: "1.1.",
)

#show heading: it => block[
  #set text(font: "Libertinus Sans")
  #set par(first-line-indent: 0pt)
  #if it.level == 1 {
    text(size: 20pt, weight: "bold", it)
    v(1em)
  } else if it.level == 2 {
    text(size: 14pt, weight: "bold", it)
    v(0.5em)
  } else if it.level == 3 {
    text(size: 12pt, weight: "bold", it)
    v(0.3em)
  } else {
    text(size: 11pt, weight: "bold", it)
    v(0.2em)
  }
]

#show link: it => text(fill: rgb("1a5276"), underline(it))

#show quote: it => block[
  #pad(left: 2em, right: 1em)[
    #set text(style: "italic", size: 10.5pt)
    #it
  ]
]

#show raw: it => block[
  #pad(left: 1em)[
    #set text(font: "DejaVu Sans Mono", size: 9pt)
    #it
  ]
]

// MARKTASTIC_BODY_CONTENT
