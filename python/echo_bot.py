from deltachat import account_hookimpl, run_cmdline

class EchoPlugin:
    @account_hookimpl
    def ac_incoming_message(self, message):
        print("process_incoming message", message)
        # unconditionally accept the chat (in case it is a contact request)
        message.create_chat()

        if not message.is_system_message() and not message.get_sender_chat().is_group():
            text = message.text
            message.chat.send_text(text)

    @account_hookimpl
    def ac_message_delivered(self, message):
        print("ac_message_delivered", message)

    # all available hooks can be found
    # in https://github.com/deltachat/deltachat-core-rust/blob/master/python/src/deltachat/hookspec.py

def main(argv=None):
    # run_cmdline is a helper function that does the basic setup for you,
    # it is defined in https://github.com/deltachat/deltachat-core-rust/blob/master/python/src/deltachat/__init__.py#L45
    run_cmdline(argv=argv, account_plugins=[EchoPlugin()])


if __name__ == "__main__":
    main()
