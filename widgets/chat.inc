  // Session values intentionally do not persist. No transcript is written to disk.
  state chatOpen = false
  state chatDraft = ""
  state chatSubmit = ""
  state chatPrompt = ""
  state chatAnswer = ""
  state chatError = ""
  state chatHistory = []
  state chatShare = false
  state chatBusy = false
  state chatSerial = 0
  state chatId = ""
  state chatSpoken = false
  state chatAudio = ""
  state chatSpeechId = ""
  state chatFocus = ""
  state chatRevision = ""
  state chatTargetKey = ""
  state chatTargetRevision = ""
  source chatClock = clock every 1s
  source chatStatus = cmd("export PATH=\"$HOME/.local/bin:$PATH\"; miilo-owl chat-status") every 30s timeout 5s status
  let widgetChatSupported = chatStatus.ok && parseJson(chatStatus.out).widgetApi == 1
  let widgetChatReady = widgetChatSupported && parseJson(chatStatus.out).ready
  source said = android.speech
  let chatCurrentKey = chatShare && focused != null ? focused._key : ""
  let chatCurrentRevision = chatShare && focused != null ? focused._revisionKey : ""
  when chatOpen && (chatTargetKey != chatCurrentKey || chatTargetRevision != chatCurrentRevision) {
    chatSubmit = ""; chatBusy = false; chatId = ""; chatHistory = []; chatAnswer = ""; chatPrompt = ""; chatError = ""; chatAudio = ""
    chatTargetKey = chatCurrentKey; chatTargetRevision = chatCurrentRevision
    speech("cancel"); sound("stop")
  }
  let chatCommand = chatOpen && chatBusy && !setupOpen && widgetChatReady ? "export PATH=\"$HOME/.local/bin:$PATH\"; printf '%s\\0' ${sh(chatId)} ${sh(chatPrompt)} ${sh(str(chatShare))} ${sh(chatFocus)} ${sh(chatRevision)} ${join(chatHistory.map(m => sh(m.role) + ' ' + sh(sub(m.text, 0, 1000))), ' ')} | miilo-owl widget-chat" : null
  source chatReply = stream(chatCommand) json
  on chatReply {
    chatAnswer = chatBusy && chatReply.requestId == chatId ? (chatReply.text ?? "") : chatAnswer
    chatError = chatBusy && chatReply.requestId == chatId ? (chatReply.error ?? "") : chatError
    chatHistory = chatBusy && chatReply.requestId == chatId && chatReply.done && chatError == "" ? drop(chatHistory, max(0, len(chatHistory) - 2)) + [{role: "user", text: chatPrompt}, {role: "assistant", text: chatAnswer}] : chatHistory
    chatBusy = chatBusy && !(chatReply.requestId == chatId && chatReply.done)
  }
  when chatOpen && !setupOpen && chatSpoken && chatSpeechId != chatId && chatReply.requestId == chatId && chatReply.done && chatReply.error == null {
    chatSpeechId = chatId
    chatAudio = run("export PATH=\"$HOME/.local/bin:$PATH\"; printf '%s\\0%s' ${sh(chatId)} ${sh(chatAnswer)} | miilo-owl widget-speak")
  }
  let audioResult = parseJson(chatAudio)
  when chatOpen && !setupOpen && chatSpoken && audioResult.requestId == chatId && audioResult.path != null { sound(audioResult.path) }
  on said.final { chatDraft = chatOpen && !chatBusy && len(value) > 0 ? sub(value, 0, 4000) : chatDraft }
  when chatOpen && !setupOpen && widgetChatReady && !chatBusy && len(chatSubmit) > 0 && len(chatSubmit) <= 4000 {
    speech("cancel"); chatSerial = chatSerial + 1; chatId = "${widgetChatPrefix}-${int(chatClock)}-${chatSerial}"
    chatPrompt = chatSubmit; chatSubmit = ""; chatDraft = ""; chatAnswer = ""; chatError = ""; chatAudio = ""
    chatFocus = chatShare && focused != null ? focused._key : ""
    chatRevision = chatShare && focused != null ? focused._revisionKey : ""
    chatBusy = true
  }
  component ChatSend() {
    Box {
      width: 44; height: 44; radius: 22; align: center
      background: alpha(accent, 0.12)
      onTap: chatSubmit = trim(chatDraft)
      Canvas { width: 22; height: 22
        path([[11, 19], [11, 3], [5, 9]], stroke: foreground, width: 1.6, cap: round)
        line(11, 3, 17, 9, stroke: foreground, width: 1.6, cap: round)
      }
    }
  }
  component ChatPanel() {
    Column {
      fill: width; gap: 12
      Row { fill: width; align: center; gap: 8
        Box { width: 44; height: 44; align: center; onTap: { chatOpen = false; chatBusy = false; speech("cancel"); sound("stop") }
          ToolbarIcon { kind: "back" }
        }
        BrandSymbol { w: 30 }
        Text { text: "Ask Miilo"; fontSize: 16; weight: 1 }
      }
      Text { text: "Questions go to your configured model. Chat uses shared context and cannot run agent actions."; style: small; color: muted; maxLines: 4 }
      Row { fill: width; wrap: true; gap: 8
        if !chatBusy {
          Button { text: chatShare ? "Share context: On" : "Share context: Off"; onTap: { chatShare = !chatShare; chatHistory = []; chatAnswer = ""; chatPrompt = "" } }
          Button { text: "Model settings"; onTap: { speech("cancel"); setupOpen = true; setupSection = "chat" } }
          if said.available != null { Button { text: chatSpoken ? "Spoken replies: On" : "Spoken replies: Off"; onTap: { chatSpoken = !chatSpoken; chatAudio = ""; sound("stop") } } }
        }
      }
      if chatShare { Text { text: focused != null ? "Sharing this work item and its evidence." : "Sharing up to 20 current work items and bounded evidence."; style: small; color: muted; maxLines: 3 } }
      for message in chatHistory {
        Column { fill: width; gap: 4
          Text { text: message.role == "user" ? "You" : "Miilo"; style: small; color: muted }
          Text { text: message.text; style: body }
        }
      }
      if chatBusy || chatError != "" {
        Text { text: chatPrompt; style: body }
        Text { text: chatAnswer == "" && chatBusy ? "Thinking…" : chatAnswer; style: body }
      }
      if chatError != "" { Text { text: chatError; style: small; color: red } }
      if chatSpoken && audioResult.requestId == chatId && audioResult.error != null { Text { text: audioResult.error; style: small; color: red } }
      if said.error != null && chatOpen { Text { text: said.error; style: small; color: red } }
      if said.listening { Text { text: said.text == "" ? "Listening…" : said.text; style: body; color: accent } }
      if chatStatus == null {
        Text { text: "Checking helper…"; style: small; color: muted }
      } else if !widgetChatSupported {
        Text { text: "Install a matching helper before using Ask Miilo."; style: small; color: muted; maxLines: 3 }
        HelperInstall { }
      } else if !widgetChatReady {
        Text { text: "Configure a chat model in Model settings to start a conversation."; style: small; color: muted; maxLines: 3 }
      } else if chatBusy {
        Button { text: "Stop reply"; onTap: { chatBusy = false; chatError = "Reply stopped. You can send another question."; sound("stop") } }
      } else {
        Row { fill: width; align: center; gap: 8
          Box { weight: 1
            Input { text: chatDraft; placeholder: "Ask about your work"; onChange: chatDraft = sub(value, 0, 4000); onSubmit: chatSubmit = trim(sub(value, 0, 4000)) }
          }
          if said.available {
            Box { width: 44; height: 44; radius: 22; align: center; background: said.listening ? alpha(accent, 0.2) : transparent
              onTap: speech("toggle")
              Canvas { width: 22; height: 22
                rect(8, 2, 6, 12, radius: 3, stroke: foreground, width: 1.4)
                path([[4, 10], [4, 14], [7, 17], [15, 17], [18, 14], [18, 10]], stroke: foreground, width: 1.4)
                line(11, 17, 11, 21, stroke: foreground, width: 1.4)
              }
            }
          }
          if len(trim(chatDraft)) > 0 && len(chatDraft) <= 4000 { ChatSend { } }
        }
        if len(chatHistory) > 0 { Button { text: "New conversation"; onTap: { chatHistory = []; chatAnswer = ""; chatPrompt = ""; chatDraft = ""; chatError = ""; speech("cancel"); sound("stop") } } }
      }
    }
  }
