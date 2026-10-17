// Isolated Qt fixture capture. Does not launch or qualify an Omarchy shell.
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickWindow>
#include <QQuickItem>
#include <QTest>
#include <QFile>
#include <QDir>
#include <QJsonDocument>
#include <QImage>
#include <QJSValue>
#include <QTextStream>
#include <stdexcept>
#include <functional>

static void require(bool value, const char *message) {
    if (!value) throw std::runtime_error(message);
}
static QByteArray read(const QString &name) {
    QFile file(name); require(file.open(QIODevice::ReadOnly), "Cannot read fixture input");
    return file.readAll();
}
static QQuickItem *find(QQuickItem *item, const std::function<bool(QQuickItem*)> &matches) {
    if (matches(item)) return item;
    for (auto child : item->childItems()) if (auto hit = find(child, matches)) return hit;
    return nullptr;
}
static void click(QQuickWindow *window, QQuickItem *item) {
    require(item, "No interactive row in rendered fixture");
    auto point = item->mapToScene(item->boundingRect().center());
    QTest::mouseClick(window, Qt::LeftButton, Qt::NoModifier, point.toPoint());
    QTest::qWait(100);
}
int main(int argc, char **argv) {
    if (qEnvironmentVariableIsEmpty("QT_QPA_PLATFORM")) qputenv("QT_QPA_PLATFORM", "offscreen");
    if (qEnvironmentVariableIsEmpty("QT_QUICK_BACKEND")) qputenv("QT_QUICK_BACKEND", "software");
    qputenv("QT_QUICK_CONTROLS_STYLE", "Basic");
    QGuiApplication app(argc, argv);
    try {
        auto args=app.arguments();
        require(args.size()==7,"Usage: miilo-preview MODE FIXTURE OUTPUT ROOT DATA SOURCE");
        const auto mode=args[1], fixture=args[2], output=args[3], root=args[4];
        require(mode=="orbit" || mode=="crew", "Unsupported preview mode");
        require(QDir().mkpath(output), "Cannot create preview output");
        QQmlApplicationEngine engine;
        QStringList warnings;
        QObject::connect(&engine, &QQmlApplicationEngine::warnings, [&](const QList<QQmlError> &items) {
            for(const auto &item:items) warnings << item.toString();
        });
        engine.addImportPath(root+"/tests/qml/stubs");
        if(mode=="orbit") {
            engine.rootContext()->setContextProperty("widgetSource",QString::fromUtf8(read(args[6])));
            QJsonParseError error;
            auto data=QJsonDocument::fromJson(read(args[5]),&error);
            require(error.error==QJsonParseError::NoError && data.isObject(),"Invalid fixture JSON");
            engine.rootContext()->setContextProperty("sceneData",data.toVariant());
            const auto paletteFile=qEnvironmentVariable("MIILO_PREVIEW_PALETTE");
            engine.rootContext()->setContextProperty("sceneTheme",paletteFile.isEmpty()?QString():QString::fromUtf8(read(paletteFile)));
            engine.rootContext()->setContextProperty("sceneView",qEnvironmentVariable("MIILO_PREVIEW_VIEW","orbit"));
            engine.rootContext()->setContextProperty("sceneLight",qEnvironmentVariable("MIILO_PREVIEW_THEME")=="light");
            engine.rootContext()->setContextProperty("sceneMotion",qEnvironmentVariable("MIILO_PREVIEW_MOTION","full"));
            auto dimension=[](const char *key,int fallback) {
                if(qEnvironmentVariableIsEmpty(key))return fallback;
                bool ok=false;int value=qEnvironmentVariableIntValue(key,&ok);
                require(ok&&value>=100&&value<=4096,"Invalid preview dimensions");return value;
            };
            engine.setInitialProperties({{"width",dimension("MIILO_PREVIEW_WIDTH",900)},
                                         {"height",dimension("MIILO_PREVIEW_HEIGHT",850)}});
        }
        engine.load(QUrl::fromLocalFile(fixture));
        require(!engine.rootObjects().isEmpty(),"Qt fixture failed to load");
        auto window=qobject_cast<QQuickWindow*>(engine.rootObjects().first());
        require(window,"Fixture must be a QQuickWindow");
        QTest::qWait(500);
        auto capture=[&](const QString &name) {
            auto image=window->grabWindow();
            require(!image.isNull(),"Empty Qt render");
            require(image.save(output+"/"+name),"Cannot save Qt render");
            return image;
        };
        if(mode=="orbit") {
            auto first=capture("orbit.png");
            auto advance=[&](double time) {
                require(QMetaObject::invokeMethod(window,"advance",Qt::DirectConnection,Q_ARG(QVariant,QVariant(time))),"Fixture advance failed");
                QTest::qWait(25);
            };
            advance(1.4);auto next=capture("orbit-next-frame.png");
            if(qEnvironmentVariable("MIILO_PREVIEW_MOTION","full")!="off")require(first!=next,"Animated fixture frames are identical");
            if(qEnvironmentVariable("MIILO_PREVIEW_RECORD")=="1") {
                require(QDir().mkpath(output+"/frames"),"Cannot create frames directory");
                bool ok=true;
                const int seconds=qEnvironmentVariableIsEmpty("MIILO_PREVIEW_SECONDS")?20:qEnvironmentVariableIntValue("MIILO_PREVIEW_SECONDS",&ok);
                require(ok&&seconds>=2&&seconds<=20,"Preview duration must be 2..20 seconds");
                for(int index=0;index<=seconds*12;++index) {
                    if(index==24 && qEnvironmentVariable("MIILO_PREVIEW_VIEW")=="inspect") {
                        click(window,find(window->contentItem(),[](QQuickItem *item){return item->property("text").toString()=="Review summary";}));
                        require(!window->property("selection").toString().isEmpty(),"Inspection click failed");
                    }
                    advance(index/12.0);capture(QString("frames/%1.png").arg(index,4,10,QChar('0')));
                }
            }
            if(qEnvironmentVariable("MIILO_PREVIEW_VIEW")=="quiet" || qEnvironmentVariable("MIILO_PREVIEW_VIEW")=="quiet-expanded") {
                window->setHeight(680);QTest::qWait(100);
                click(window,find(window->contentItem(),[](QQuickItem *item){return item->property("text").toString()=="Automation factory";}));
            }
            if(window->property("selection").toString().isEmpty()) {
            auto label=find(window->contentItem(),[](QQuickItem *item) {
                return item->property("text").toString()=="Review summary";
            });
            click(window,label);
            require(!window->property("selection").toString().isEmpty(),"Work selection did not open details");
            }
            for(const auto &text: {"Files & changes","activity-summary.ts","Verification output"}) {
                require(find(window->contentItem(),[&](QQuickItem *item){return item->property("text").toString()==text;}),"Expected evidence or output was not rendered");
            }
            capture("orbit-detail.png");
            auto errors=window->property("errors");
            require(errors.canConvert<QJSValue>() && errors.value<QJSValue>().property("length").toInt()==0,"OWL runtime errors");
        } else {
            for(const auto &name: {"compactAction", "compactField", "compactChoice"}) {
                auto control=window->findChild<QQuickItem*>(name);
                require(control,"Missing compact control in native fixture");
                const auto top=control->property("topPadding").toDouble(), bottom=control->property("bottomPadding").toDouble();
                require(top<=4&&bottom<=4,"Control vertical padding was not reduced");
                require(control->height()>=control->property("contentHeight").toDouble(),"Compact control clips text");
            }
            capture("crew-dark.png");
            auto row=find(window->contentItem(),[](QQuickItem *item) {
                auto expanded=item->property("expanded");return expanded.isValid()&&!expanded.toBool();
            });
            click(window,row);
            require(find(window->contentItem(),[](QQuickItem *item){return item->property("expanded").toBool();}),"Agent row did not expand");
            capture("crew-expanded.png");
            window->setProperty("light",true);window->setWidth(360);QTest::qWait(100);
            capture("crew-light-narrow.png");
        }
        require(warnings.isEmpty(),qPrintable(warnings.join('\n')));
        QTextStream(stdout)<<"PASS: native Qt fixture, captures and real row selection. Synthetic data; no shell lifecycle or performance claim.\n";
        return 0;
    } catch(const std::exception &error) {
        QTextStream(stderr)<<error.what()<<'\n';return 1;
    }
}
