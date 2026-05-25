// University Assignment Template
// Assignment-style with title block and generous margins

#set page(
  paper: "a4",
  margin: (top: 3cm, bottom: 3cm, left: 3cm, right: 3cm),
  numbering: "1",
)

#set text(
  font: "New Computer Modern",
  size: 12pt,
  lang: "en",
)

#set heading(
  numbering: none,
)

#show heading: it => block[
  #set text(font: "New Computer Modern")
  #if it.level == 1 {
    text(size: 16pt, weight: "bold", it)
    v(0.8em)
    line(length: 100%, stroke: 0.5pt + gray)
    v(0.5em)
  } else if it.level == 2 {
    text(size: 13pt, weight: "bold", it)
    v(0.4em)
  } else {
    text(size: 11pt, weight: "bold", it)
    v(0.2em)
  }
]

#show link: it => text(fill: rgb("1a5276"), underline(it))

#show quote: it => block[
  #pad(left: 1.5em, right: 1em)[
    #set text(style: "italic")
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
