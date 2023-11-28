#let template(body) = {
  set text(12pt, font: "Fasthand")
  set page(
    paper: "us-letter",
    margin: (left: 1.6cm, right: 1.6cm, top: 1.5cm),
    fill: blue
  )
  body
}