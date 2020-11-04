# Quist

Quickly create short-lived Gists based on local files or existing Gists.

## Table of contents
1. [Usage](#usage)

## Usage
First, you need to [create a personal token](https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/creating-a-personal-access-token) for creating Gists. **Storing the token safely is up to you.**

After that, run:

```console
$ GIST_TOKEN=b1f63f8bed1b29584bd4ef0d7437254f0d069ced # this is an example, don't worry
$ quist --basic-auth=gbrlsnchs:$GIST_TOKEN welcome.txt
Gist created at https://gist.github.com/gbrlsnchs/0db1a400ee13076c1646cbca82b9b830
Waiting for SIGINT...
^C
Gist "aa5a315d61ae9438b18d" has been successfully deleted
```

Note that this is not a CLI to manage Gists, which you could do with the official GitHub CLI. This is just a simple utility tool to create short-lived Gists.
