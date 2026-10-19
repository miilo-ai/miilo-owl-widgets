# Observe another computer

Install Miilo CLI 0.4.0 or newer on the sharing computer and widget helper 1.7.0
or newer on the receiver. Both devices need a working network route. A private
network such as Tailscale is suitable; no cloud account or public listener is
required. The computer must remain awake and collecting its selected sessions.

On the computer, select sessions with `miilo local setup`, run `miilo local run`,
and start sharing in another terminal:

```sh
miilo local share --bind PRIVATE_IP:38761 --url https://PRIVATE_IP:38761 --name "Workstation"
```

Run `miilo local pair invite` there and privately transfer the complete invitation.
In the widget, open **Settings → Pair computer**, choose **Pair computer**, paste
it at the hidden prompt and name the receiving device. On the computer, run
`miilo local pair requests`. Compare the six-digit code displayed on both devices,
then approve with `miilo local pair approve REQUEST_ID COMPARISON_CODE`.
Select the systems to display at the receiver's final prompt. Idle selected systems
remain sleeping robots; pairing does not make them active or launch agent work.

To display newly selected sessions from an already-paired computer, open
**Connections → Manage connections**. Choose `a` and enter the paired endpoint
identifier shown in the list, then select its systems. This preserves the pinned
identity and device credential; it does not pair again or enable actions.

Invitations are single-use and expire after five minutes. Device credentials last
30 days. Create another invitation if one expires; synchronize device clocks if
reported. A small clock skew is tolerated without extending the host's deadline.
Use `miilo local pair devices` and `miilo local pair revoke DEVICE_ID` on the host
to remove access. Removing the widget connection alone does not revoke a copied
credential. Stop the sharing process with Ctrl+C to stop all remote reads.

Explain presents the supplied observation summary and its limitations; it does not
invoke a model on the host. Your own chat endpoint remains a separate configuration.

Only selected observation snapshots, including their summaries/evidence, are
shared. No agent login, model credential, approval or execution authority is
forwarded. Each receiver has a separate credential. The host certificate is pinned;
a change fails closed. Widget credentials are stored in its existing owner-only
configuration directory, not in widget source. File permissions protect local
access; they do not encrypt backups. Keep device backups private.
