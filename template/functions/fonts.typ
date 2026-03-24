#let line_height = 1em

#let fonts = (
  serif: (
    "New Computer Modern",
    "SimSun",
  ),
  sans: (
    "New Computer Modern",
    "SimHei",
  ),
  monospace: (
    "MesloLGS NF",
    "SimSun",
  ),
  italic: (
    "New Computer Modern",
    "KaiTi",
  )
)

// 加粗：自动回退到 sans 数组，即英文 NewCM Bold + 中文黑体
#let textbf(it) = text(font: fonts.sans, weight: "bold", it)

// 斜体：自动回退到 italic 数组，即英文 NewCM Italic + 中文楷体
#let textit(it) = text(font: fonts.italic, style: "italic", it)