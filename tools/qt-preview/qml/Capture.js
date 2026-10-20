.pragma library
function find(item, matches) {
    if (matches(item)) return item
    var children = item.children || []
    for (var i = 0; i < children.length; ++i) {
        var found = find(children[i], matches)
        if (found) return found
    }
    return null
}
function text(scene, value) { return find(scene.contentItem, function(item) { return item.text === value }) }
function click(test, item) {
    test.verify(item !== null, "Missing interactive fixture item")
    test.mouseClick(item, item.width / 2, item.height / 2)
    test.wait(100)
}
function capture(test, scene, config, name) {
    scene.requestUpdate()
    test.wait(25)
    var image = test.grabImage(scene.contentItem)
    test.verify(image.width > 0 && image.height > 0, "Empty Qt render")
    image.save(config.output + "/" + name)
    return image
}
