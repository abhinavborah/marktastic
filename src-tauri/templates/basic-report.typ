// Basic Report Template
// Clean academic report style with page numbers and headings

#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 2.5cm, right: 2.5cm),
  numbering: "1",
  header: align(right, text(size: 9pt, fill: gray)[Marktastic Report]),
  footer: align(center, context { counter(page).display("1") }),
)

#set text(
  font: "Libertinus Serif",
  size: 11pt,
  lang: "en",
)

#set heading(
  numbering: "1.1.",
)

#show heading: it => block[
  #set text(font: "Libertinus Sans")
  #if it.level == 1 {
    text(size: 18pt, weight: "bold", it)
    v(0.5em)
  } else if it.level == 2 {
    text(size: 14pt, weight: "bold", it)
    v(0.3em)
  } else {
    text(size: 12pt, weight: "bold", it)
    v(0.2em)
  }
]

#show link: it => text(fill: blue, underline(it))

#show quote: it => block[
  #pad(left: 1em)[
    #set text(style: "italic")
    #it
  ]
]

// MARKTASTIC_BODY_CONTENT
