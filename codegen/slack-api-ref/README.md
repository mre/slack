Slack API Reference
-------------------

![](slack.png)

This is a maintained, machine-readable version of the Slack API Docs, generated by scraping [api.slack.com](https://api.slack.com).

* [groups](groups): Web API groups.
* [methods](methods): Web API methods, organized in groups.
* [events](events): RealTime API events.

### It needs an update!

Run `rake api:update`, make a pull request. See [CONTRIBUTING](CONTRIBUTING.md) for details.

### Usage

The reference is used by the following client projects to generate API client code.

* [slack-ruby-client](https://github.com/dblock/slack-ruby-client): Slack Ruby Client

### Slack

This project is not affiliated with the [company that makes Slack](https://slack.com). We began by forking JSON schema files from [slackhq/slack-api-docs](https://github.com/slackhq/slack-api-docs), then updating the schema from the documentation at [api.slack.com](https://api.slack.com) to, finally, relying on a scraper.

### Copyright & License

Copyright (c) 2015-2016, Daniel Doubrovkine and [Contributors](CHANGELOG.md).

This project is licensed under the [MIT License](LICENSE.md).
