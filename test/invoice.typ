#import "@preview/invoice-maker:1.1.0": *

#let data = json(bytes(sys.inputs.data))
#let banner-image = align(center)[#image(data.banner-image)]

#show: invoice.with(..data, banner-image: banner-image)
