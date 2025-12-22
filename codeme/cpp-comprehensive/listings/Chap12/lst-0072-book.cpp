
Page page = Html().body()
    .h1("Heading")
    .table().border(0)
      .tr()
        .td().css("head").text("Dog Breed").end()
        .td().text("Poodle").end()
      .end()
    .end()
    .toPage();