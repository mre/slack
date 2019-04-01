# slack

This project aims to create a simple, [idiomatic](https://github.com/mre/idiomatic-rust), fully documented Rust library for the Slack Web API.
I gave a talk about it, titled "What's so hard about writing a Slack Client in Rust?" [[Video](https://www.youtube.com/watch?v=rrtJh1kz1Ms)] [[Slides](https://speakerdeck.com/mre/whats-so-hard-about-writing-a-slack-client-in-rust)].

## Status

All code and error handling for the Slack methods are fully auto-generated.  
What is missing is generating proper request and response types from the API.  
This would be a lot of manual work at the moment, because the Slack API
documentation is not machine-readable. There is [an open ticket](https://github.com/slackhq/slack-api-docs/issues/74) for that and some
[possible](https://github.com/slack-ruby/slack-api-ref/issues/17) [workarounds](https://github.com/slack-rs/slack-api-schemas/), but not much traction.  
Because of that I need to halt this project for the near future.  
I'm uploading it anyway to inspire fellow Rustaceans.

**Update**: There is an OpenAPI-based machine-readable spec available now. See [here](https://github.com/slackapi/slack-api-specs). Maybe someone wants to give it a shot.

## Goals

* Great ergonomics. Easy to use.
* Pure, idiomatic Rust code.
* Fully documented and tested.
* Solid error handling.
* Proper conversion from JSON to semantic types.
* Full Slack API implementation.

## Quick start

```Rust
let mut client = slack::Client::new("your_slack_token");
let response = client.im_history("D0AA48KM3").latest(123.1).send();
```

Alternatively you can build the request objects yourself:

```Rust
  let client = slack::Client::new("your_slack_token");
  let history = history::HistoryOptions {
      channel: "hello",
      inclusive: Some(true),
      ..Default::default()
  };
  println!("{}", client.send(history).unwrap());
```

## TODO

* Auto-generate Slack request and response types
* Enforce documentation (`#![deny(missing_docs)]`)

## Alternatives

https://github.com/slack-rs/slack-rs-api
