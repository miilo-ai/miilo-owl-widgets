import dev.omalauncher.widgets.lang.*
import java.io.File

// Real OWL expressions and handlers, without executing provider shell commands.
private class FixtureHost(val program: Program, fixture: String, val width: Double, val height: Double, themeName: String, scenario: String) : Host {
    val slots = Array<Value>(program.globals.size) { Null }
    val memo = mutableMapOf<Int, Value>()
    val errors = mutableListOf<String>()
    val runs = mutableListOf<String>()
    var renderWidth = width - 16
    val colors = if (themeName == "light") mapOf("accent" to 0xff5e469e.toInt(), "background" to 0xfffaf5ee.toInt(), "foreground" to 0xff28232e.toInt(), "muted" to 0xff776575.toInt(), "border" to 0xffc8b9cb.toInt(), "red" to 0xffb52f47.toInt()) else mapOf("accent" to 0xffac9cf5.toInt(), "background" to 0xff16151d.toInt(), "foreground" to 0xffeeeaf4.toInt(), "muted" to 0xff9993a8.toInt(), "border" to 0xff37333f.toInt(), "red" to 0xffed8b95.toInt())
    init {
        program.globals.forEachIndexed { i, g ->
            if (g is Global.State) slots[i] = g.init(Env.root(this))
            if (g is Global.Source && g.name == "feed") slots[i] = Obj(mapOf("ok" to TRUE, "out" to Str(File(fixture).readText())))
        }
        val data = Json.parse(File(fixture).readText()) as Obj
        val snap = data.fields["snapshot"] as Obj
        fun first(name: String) = ((snap.fields[name] as? Lst)?.items?.firstOrNull() as? Obj)
        fun state(name: String, value: Value) { program.globals.indexOfFirst { it.name == name }.takeIf { it >= 0 }?.let { slots[it] = value } }
        val item = first("workItems")
        if (scenario in listOf("detail", "confirm", "interaction") && item != null) {
            state("selectedWork", item.fields["_key"] ?: Null)
            state("evidenceOpen", TRUE)
            if (scenario == "confirm") {
                val action = (item.fields["actions"] as? Lst)?.items?.firstOrNull() as? Obj
                state("proposed", action?.fields?.get("_key") ?: Str(""))
                state("proposalWork", item.fields["_key"] ?: Null)
                state("proposalRevision", item.fields["_revisionKey"] ?: Null)
            }
        }
        if (scenario == "filter") state("system", first("providers")?.fields?.get("id") ?: Str(""))
        if (scenario == "actor") state("selectedActor", first("actors")?.fields?.get("_key") ?: Str(""))
    }
    override fun read(slot: Int): Value = (program.globals[slot] as? Global.Let)?.let { memo.getOrPut(slot) { it.value(Env.root(this)) } } ?: slots[slot]
    override fun write(slot: Int, value: Value) { slots[slot] = value; memo.clear() }
    override fun theme(name: String): Int? = colors[name]
    override fun tile(name: String): Value = Num(if (name == "tileHeight") height else width)
    override fun time() = 1.5
    override fun act(name: String, args: List<Value>) {}
    override fun runInto(command: String, slot: Int) { runs += command }
    override fun report(message: String) { errors += message }
    override val budget = Budget()
}

private fun expand(nodes: List<WNode>, env: Env): List<Value> = nodes.flatMap { node ->
    when (node) {
        is WNode.If -> expand(if (node.cond(env).truthy) node.then else node.orElse, env)
        is WNode.For -> (node.list(env) as? Lst)?.items.orEmpty().flatMapIndexed { i, item -> expand(node.body, env.child(item, Num(i.toDouble()))) }
        is WNode.Lets -> expand(node.children, env.lets(node.values))
        is WNode.Use -> expand(node.body, env.child(*node.args.map { it(env) }.toTypedArray()))
        is WNode.Element -> {
            val props = node.props.mapValues { it.value(env) }
            val e = if (node.kind == "Canvas") env.child(props["width"] ?: Num((env.host as FixtureHost).renderWidth.coerceAtLeast(20.0)), props["height"] ?: Num(40.0)) else env
            listOf(Obj(mapOf("kind" to Str(node.kind), "props" to Obj(if (node.kind == "Canvas") props + ("_drawWidth" to (props["width"] ?: Num((env.host as FixtureHost).renderWidth))) else props), "children" to Lst(expand(node.children, e)))))
        }
        is WNode.Draw -> listOf(Obj(mapOf("op" to Str(node.op), "args" to Lst(node.args.map { it(env) }), "named" to Obj(node.named.mapValues { it.value(env) }))))
    }
}

private fun click(nodes: List<WNode>, env: Env, label: String): Boolean {
    for (node in nodes) {
        val found = when(node) {
            is WNode.If -> click(if(node.cond(env).truthy) node.then else node.orElse, env, label)
            is WNode.For -> (node.list(env) as? Lst)?.items.orEmpty().withIndex().any { (i, item) -> click(node.body, env.child(item, Num(i.toDouble())), label) }
            is WNode.Lets -> click(node.children, env.lets(node.values), label)
            is WNode.Use -> click(node.body, env.child(*node.args.map { it(env) }.toTypedArray()), label)
            is WNode.Element -> if (node.kind == "Button" && node.props["text"]?.invoke(env)?.toString() == label) {
                node.handlers["onTap"]!!.run(env, listOf(Num(0.0), Num(0.0))); true
            } else if(node.kind != "Canvas") click(node.children, env, label) else false
            is WNode.Draw -> false
        }
        if(found) return true
    }
    return false
}

fun main(args: Array<String>) {
    val p = Compiler.compile(File(args[0]).readText())
    val host = FixtureHost(p, args[1], args[2].toDouble(), args[3].toDouble(), args[4], args[5])
    if(args[5] == "interaction") {
        check(click(p.screen.orEmpty(), Env.root(host), "Approve"))
        check(host.runs.isEmpty()) { "First tap must only propose" }
        check(click(p.screen.orEmpty(), Env.root(host), "Confirm"))
        check(host.runs.size == 1 && host.runs[0].startsWith("miilo-owl action "))
    }
    val tile = host.budget.run { expand(p.root, Env.root(host)) }
    host.renderWidth = 328.0
    val screen = host.budget.run { expand(p.screen.orEmpty(), Env.root(host)) }
    check(host.errors.isEmpty()) { host.errors.joinToString("\n") }
    fun jsonValue(v: Value): Value = when(v) {
        is Col -> Str("#%08x".format(Builtins.argb(v, host) ?: 0))
        is Obj -> Obj(v.fields.mapValues { jsonValue(it.value) })
        is Lst -> Lst(v.items.map { jsonValue(it) })
        else -> v
    }
    println(Json.write(jsonValue(Obj(mapOf("tile" to Lst(tile), "screen" to Lst(screen))))))
}
