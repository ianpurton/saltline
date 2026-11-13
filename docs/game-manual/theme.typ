#let ink = rgb("27343c")
#let blue = rgb("50758a")
#let amber = rgb("b77916")
#let rust = rgb("a94c30")
#let bone = rgb("f5f1e8")
#let manual(body) = {
 set document(title: "SALTLINE — Game Construction Manual", author: "SALTLINE reference project")
 set page(paper: "a4", margin: 14mm, fill: bone,
  header: [#text(size: 8pt, fill: blue, weight: "bold")[SALTLINE / CONSTRUCTION REFERENCE] #h(1fr) #text(size: 8pt)[HTML → engine-independent specification]],
  footer: [#text(size: 7.5pt, fill: blue)[SOURCE-DERIVED RULES • SCHEMATIC VECTORS] #h(1fr) #context counter(page).display("01")])
 set text(font: "Liberation Sans", size: 9.5pt, fill: ink)
 set par(justify: true, leading: .6em)
 set heading(numbering: "1.1")
 show heading.where(level: 1): it => {v(2pt);text(size: 23pt, weight: "bold", fill: ink, it.body);v(4pt)}
 show heading.where(level: 2): it => {v(3pt);text(size: 12pt, weight: "bold", fill: blue, it.body);v(2pt)}
 show raw: set text(font: "Liberation Mono", size: 8pt)
 set table(inset: 4pt, stroke: .3pt + rgb("d3cec4"), fill: (_, y) => if calc.odd(y) {rgb("ece8df")} else {none})
 body
}
#let cols(body) = columns(2, gutter: 6mm, body)
#let note(body) = block(fill: rgb("e8e5dc"), inset: 7pt, width: 100%, text(size: 8.5pt, body))
#let plate(file, caption, height: auto) = figure(image("assets/" + file, width: 100%, height: height, fit: "contain"), caption: text(size: 8pt, caption))
#let sheet(n, title, subtitle, body) = {pagebreak(weak: true);text(size: 8pt, fill: amber, weight: "bold")[FIELD SHEET #n];heading(level: 1, title);text(size: 9pt, fill: blue, subtitle);v(5pt);body}
#let facts(data, keys, names: (:)) = table(columns: (1.4fr, 1fr), table.header([*Parameter*], [*Source value*]), ..keys.map(k => ([#names.at(k, default: k)], [#str(data.at(k))])).flatten())
#let ref(name, data) = {
 let a=data.anchors.at(name)
 text(size: 7.5pt, fill: blue)[Source: #name · line #a.line · search #raw(a.anchor)]
}
