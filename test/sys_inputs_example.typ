#let persons = json(bytes(sys.inputs.persons))

#for person in persons [
  #person.name is #person.age years old.\
]
