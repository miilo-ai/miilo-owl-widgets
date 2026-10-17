// Miilo host typography. Logical units; Qt owns display-density conversion.
// Host text preferences are independent of widget/grid/robot geometry.
var DESKTOP = { huge: 32, title: 16, body: 13, small: 12, muted: 13, label: 12 };
var PHONE = { huge: 34, title: 20, body: 16, small: 14, muted: 16, label: 14 };

function validScale(value) {
    var number = Number(value);
    return isFinite(number) && number > 0 ? number : 1;
}

function size(role, profile, scale) {
    var roles = profile === 'phone' ? PHONE : DESKTOP;
    var base = typeof roles[role] === 'number' ? roles[role] : roles.body;
    return Math.max(1, Math.round(base * validScale(scale)));
}

function leading(role) {
    return role === 'title' || role === 'huge' ? 1.25 : 1.45;
}
