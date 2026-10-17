"""Render actual OWL/QML with synthetic inventory; no commands or shell services.
Requires developer-only PySide6-Essentials. This is not Omarchy shell qualification.
"""
import os
import shutil
import sys
import tempfile
from pathlib import Path

os.environ.setdefault('QT_QPA_PLATFORM', 'offscreen')
os.environ.setdefault('QT_QUICK_BACKEND', 'software')
from PySide6.QtCore import QPoint, Qt, QTimer, QUrl, QMetaObject, Q_ARG
from PySide6.QtGui import QGuiApplication
from PySide6.QtQuick import QQuickWindow
from PySide6.QtQml import QQmlApplicationEngine
from PySide6.QtTest import QTest

root = Path(__file__).resolve().parents[1]
output = Path(sys.argv[1] if len(sys.argv) > 1 else '/tmp/miilo-orbit-preview')
output.mkdir(parents=True, exist_ok=True)
temp = tempfile.TemporaryDirectory(prefix='miilo-orbit-qt-')
folder = Path(temp.name)
(folder / 'runtime').mkdir()
for name in ['Owl.js', 'OwlLayout.js', 'OwlNode.qml', 'OwlCanvas.qml', 'MiiloTypography.qml', 'MiiloTypeScale.js']:
    shutil.copy2(root / 'desktop/runtime' / name, folder / 'runtime' / name)
# Only the unused terminal adapter is stubbed. Actual geometry/layout/paint is used.
(folder / 'runtime/OwlTerminal.qml').write_text('import QtQuick\nItem { property var node; property var widget }\n')
shutil.copy2(root / 'tests/desktop/OrbitFixture.qml', folder / 'orbit.qml')
providers = [{'id': 'demo-factory', 'name': 'Automation factory'}, {'id': 'demo-agent', 'name': 'Coding sessions'}, {'id': 'demo-rest', 'name': 'Resting system'}]
actors = [{'_key': p['id'] + '-actor', 'id': 'actor', 'providerId': p['id'], 'name': 'Builder', 'status': 'resting' if p['id'] == 'demo-rest' else 'working', 'health': 'healthy'} for p in providers]
work = [{'_key': p['id'] + str(i), 'id': str(i), 'providerId': p['id'], 'title': title,
         '_actorKeys': [p['id'] + '-actor'], 'lifecycle': 'active', 'phase': 'verifying' if i == 0 else 'implementing',
         'attention': 'review' if i == 0 else 'none', '_revisionKey': 'r1', 'actions': [], 'summary': 'Synthetic visual fixture.', 'nativeUrl': ''}
        for p in providers if p['id'] != 'demo-rest' for i, title in enumerate(['Review streaming checks', 'Improve workspace setup', 'Validate recovery'])]
data = {'configured': True, 'errors': [], 'snapshot': {'providers': providers, 'actors': actors, 'workItems': work, 'attention': []}}
app = QGuiApplication(sys.argv)
engine = QQmlApplicationEngine()
problems = []
engine.warnings.connect(lambda items: problems.extend(item.toString() for item in items))
engine.rootContext().setContextProperty('widgetSource', (root / '70-miilo-fleet.owl').read_text())
engine.rootContext().setContextProperty('sceneData', data)
engine.rootContext().setContextProperty('sceneMotion', os.environ.get('MIILO_PREVIEW_MOTION', 'full'))
engine.rootContext().setContextProperty('sceneLight', os.environ.get('MIILO_PREVIEW_THEME') == 'light')
engine.setInitialProperties({'width': int(os.environ.get('MIILO_PREVIEW_WIDTH', '900')), 'height': int(os.environ.get('MIILO_PREVIEW_HEIGHT', '850'))})
engine.load(QUrl.fromLocalFile(str(folder / 'orbit.qml')))
if not engine.rootObjects():
    raise SystemExit('Orbit QML fixture failed to load')
window = engine.rootObjects()[0]


def descendants(item):
    yield item
    for child in item.childItems():
        yield from descendants(child)


def capture(name):
    image = window.grabWindow()
    assert not image.isNull(), 'Empty Orbit render'
    image.save(str(output / name))


def validate():
    try:
        capture('orbit.png')
        QMetaObject.invokeMethod(window, 'advance', Qt.DirectConnection, Q_ARG('QVariant', 1.4))
        QTest.qWait(80)
        capture('orbit-next-frame.png')
        if os.environ.get('MIILO_PREVIEW_RECORD') == '1':
            frames = output / 'frames'
            frames.mkdir(exist_ok=True)
            # Deterministic source time, not a native frame-rate benchmark.
            for index in range(241):
                QMetaObject.invokeMethod(window, 'advance', Qt.DirectConnection, Q_ARG('QVariant', index / 12))
                QTest.qWait(20)
                capture('frames/%04d.png' % index)
        label = next(i for i in descendants(window.contentItem()) if i.property('text') == 'Review streaming checks')
        point = label.mapToScene(label.boundingRect().center())
        QTest.mouseClick(window, Qt.LeftButton, Qt.NoModifier, QPoint(round(point.x()), round(point.y())))
        QTest.qWait(100)
        assert window.property('selection'), 'Native work-row selection did not open details'
        capture('orbit-detail.png')
        assert not window.property('errors').toVariant(), 'OWL runtime errors'
        assert not problems, '\n'.join(problems)
        print('PASS: actual Qt OWL layout/Canvas, controlled frames and native work selection.')
        print('Synthetic inventory, no commands; host lifecycle/performance remain separate.')
        app.exit(0)
    except Exception as error:
        print(error)
        app.exit(1)


QTimer.singleShot(500, validate)
sys.exit(app.exec())
