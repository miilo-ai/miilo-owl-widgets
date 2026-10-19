// An OWL Canvas: asks the instance for this frame's display list and paints
// it. The drawing calls are evaluated in dp, so the context is scaled once.
//
// A widget that reads `time` repaints every frame while its board is up
// (OwlWidget's frame counter); any other repaints only when the node, the
// size or the theme changes.

import QtQuick
import "OwlLayout.js" as L

Canvas {
  id: root

  property var node: null
  property var widget: null

  readonly property real dp: root.widget ? root.widget.dp : 1
  readonly property var draw: root.node ? root.node.draw : null

  // Fills the space unless sized; OwlLayout.place() gives it that space.
  implicitWidth: 0
  implicitHeight: 0

  // Qt 6 ignores FramebufferObject and uses Image. Keep operation counts
  // bounded and qualify actual frame cost; this is not a GPU-path guarantee.
  renderTarget: Canvas.Image
  renderStrategy: Canvas.Cooperative

  property bool animated: true
  property bool dirty: true
  function inViewport() {
    if (!root.visible || !root.widget || !root.widget.active || width <= 0 || height <= 0) return false
    for (var parent = root.parent; parent; parent = parent.parent) {
      if (!parent.visible || parent.opacity === 0) return false
      if (parent.clip || parent === root.widget) {
        var bounds = root.mapToItem(parent, Qt.rect(0, 0, root.width, root.height))
        if (bounds.x + bounds.width <= 0 || bounds.y + bounds.height <= 0
            || bounds.x >= parent.width || bounds.y >= parent.height) return false
      }
    }
    return true
  }
  // Track ancestor geometry so static widgets resume on scroll without a clock.
  readonly property var viewportGeometry: {
    var geometry = [root.x, root.y, root.width, root.height]
    for (var item = root.parent; item; item = item.parent)
      geometry.push(item.x, item.y, item.width, item.height, item.scale, item.rotation,
                    item.clip, item.visible, item.opacity)
    return geometry
  }
  onViewportGeometryChanged: root.schedule()
  function schedule() { if ((root.dirty || root.animated) && root.inViewport()) root.requestPaint() }
  readonly property var deps: [root.draw, root.width, root.height,
    root.widget ? root.widget.palette : null]
  onDepsChanged: { root.dirty = true; root.animated = true; root.schedule() }
  onVisibleChanged: if (visible) root.schedule()
  Connections {
    target: root.widget
    function onFrameChanged() { root.schedule() }
    function onActiveChanged() { root.schedule() }
  }

  // Images a drawing call names load asynchronously; the frame that asked
  // for one skips it and the next frame, after onImageLoaded, has it.
  readonly property var images: ({
    get: function(src) {
      var url = L.url(src, root.widget ? root.widget.home : "")
      if (!url) return null
      if (root.isImageLoaded(url)) return url
      if (!root.isImageLoading(url) && !root.isImageError(url)) root.loadImage(url)
      return null
    }
  })
  onImageLoaded: root.requestPaint()

  onPaint: {
    if (!root.inViewport()) return
    var ctx = getContext("2d")
    ctx.reset()
    if (!root.draw || width <= 0 || height <= 0) return
    ctx.scale(root.dp, root.dp)
    var ops = root.draw(width / root.dp, height / root.dp)
    root.animated = root.draw.readsTime !== false
    root.dirty = false
    L.paint(ctx, ops, { foreground: root.widget ? root.widget.ink("foreground") : "white" },
      root.widget ? root.widget.sansFamily : "sans-serif", root.images)
  }
}
