# Echo Bot - Rust

To run the bot, the first time you need to set the credentials in the
environment:

```
addr=$yourEmail mail_pw=$yourPassword cargo run
```

This will create a subdirectory called `deltachat-db` in the current
working directory, this is where deltachat stores the state.  The
credentials will be stored here so further invocations do not need
them specified again if the `deltachat-db` directory can be found
again.


### Documentation

The echo bot has been documented and can viewed using `cargo doc
--open` which will show the documentation for the `deltachat_echo_bot`
crate.  This will include the documentation of the `deltachat` crate
and all its dependencies.  But please heed the warning regarding the
partial documentation for the core in the echo bot's crate docs.

You can also find the [core sourcecode on
github](https://github.com/deltachat/deltachat-core-rust).
