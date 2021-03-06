#!/bin/sh

set -eu

git_sha() {
    git rev-parse "$1" | cut -c 1-8
}

proxy_deps_sha() {
    cat Cargo.lock proxy/Dockerfile-deps | shasum - | awk '{print $1}' |cut -c 1-8
}

go_deps_sha() {
    cat Gopkg.lock Dockerfile-go-deps | shasum - | awk '{print $1}' |cut -c 1-8
}

dir_tag() {
    dir="$1"
    echo "git-$(git log -n 1 --format="%h" "$dir")"
}

clean_head_root_tag() {
    if git diff-index --quiet HEAD -- ; then
        echo "git-$(git_sha HEAD)"
    else
        echo "Commit unstaged changes or set an explicit build tag." >&2
        exit 3
    fi
}

master_root_tag() {
    echo "git-$(git_sha master)"
}

validate_tag() {
    file="$1"
    shift

    image="$1"
    shift

    sha="$1"
    shift

    dockerfile_tag=$(grep -oe $image':[^ ]*' $file) || true
    deps_tag="$image:$sha"
    if [ "$dockerfile_tag" != "" ] && [ "$dockerfile_tag" != "$deps_tag" ]; then
        echo "Tag in "$file" does not match source tree:"
        echo $dockerfile_tag" ("$file")"
        echo $deps_tag" (source)"
        exit 3
    fi
}

# These functions should be called by any docker-build-* script that relies on
# Go or Rust dependencies. To confirm the set of scripts that should call this
# function, run:
# $ grep -ER 'docker-build-(go|proxy)-deps' .

validate_go_deps_tag() {
    file="$1"
    validate_tag "$file" "gcr.io/runconduit/go-deps" "$(go_deps_sha)"
}

validate_proxy_deps_tag() {
    file="$1"
    validate_tag "$file" "gcr.io/runconduit/proxy-deps" "$(proxy_deps_sha)"
}
