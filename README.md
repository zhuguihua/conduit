![conduit][logo]

[![Build Status][ci-badge]][ci]
[![GitHub license][license-badge]](LICENSE)
[![Slack Status][slack-badge]][slack]

:balloon: Welcome to Conduit! :wave:

Conduit is an ultralight service mesh for Kubernetes from the makers of [Linkerd][l5d]. It
features a native proxy, written in [Rust][rust], that boasts the performance of C without
all the heartbleed.

Conduit is **experimental**. Currently, it _only supports HTTP/2_ and is especially
well-suited for [gRPC][grpc]. Follow our progress towards production-readiness here and on
[Twitter][twitter].

<!-- TODO add roadmap link -->

## Documentation

View [Conduit docs][conduit-docs] for more a more comprehensive guide to
getting started.


## Getting started with Conduit

1. Install the Conduit CLI `curl https://run.conduit.io/install | sh `

2. Copy `/Users/username/.conduit/bin/conduit` into your PATH

3. Run `conduit install | kubectl apply -f -` to install Conduit

4. Check to make sure you're running the [latest version][releases]of Conduit
with `conduit version`

5. Open a local version of the Conduit controller with `conduit dashboard`

6. (Optional) To install a demo application for your Conduit instance, visit
[this section on Conduit docs][conduit-demo]


## Working in this repo ##

[`BUILD.md`](BUILD.md) includes general information on how to work in this repo.


## Code of Conduct

This project is for everyone. We ask that our users and contributors take a few
minutes to review our [code of conduct][coc].


## License

Copyright 2017, Buoyant Inc. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
these files except in compliance with the License. You may obtain a copy of the
License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.

<!-- refs -->
[ci]: https://travis-ci.org/runconduit/conduit
[ci-badge]: https://travis-ci.org/runconduit/conduit.svg?branch=master
[coc]: https://github.com/linkerd/linkerd/wiki/Linkerd-code-of-conduct
[conduit-demo]: https://conduit.io/getting-started/#install-the-demo-app
[conduit-docs]: https://conduit.io/docs/
<!-- [examples]: https://github.com/runconduit/conduit-examples -->
[grpc]: https://grpc.io/
[l5d]: https://linkerd.io/
[license-badge]: https://img.shields.io/github/license/linkerd/linkerd.svg
[logo]: https://user-images.githubusercontent.com/240738/33589722-649152de-d92f-11e7-843a-b078ac889a39.png
[releases]: https://github.com/runconduit/conduit/releases
[rust]: https://rust-lang.org/
[twitter]: https://twitter.com/runconduit/
[slack-badge]: http://slack.linkerd.io/badge.svg
[slack]: http://slack.linkerd.io
