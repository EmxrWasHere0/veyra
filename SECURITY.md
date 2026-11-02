# Security Policy

## Supported Versions

Security fixes are generally prepared for the latest published stable release. Older releases may not receive security updates.

| Component         | Current version                  | Security support                                               |
| ----------------- | -------------------------------- | -------------------------------------------------------------- |
| Veyra interpreter | 0.2.9                            | Latest published stable release                                |
| Python VPM        | 0.1.0 beta                       | Security reports are accepted; fixes are assessed case by case |
| Veyra VPM         | Experimental, development paused | No ongoing support or release guarantee                        |

## Reporting a Vulnerability

Please report suspected vulnerabilities privately. **Do not disclose vulnerability details in a public issue, discussion, or pull request.**

Use [GitHub private vulnerability reporting](https://github.com/EmxrWasHere0/veyra/security/advisories/new) to contact the maintainers.

If private reporting is unavailable, open a GitHub issue asking only for a private reporting channel. Do not include vulnerability details or proof-of-concept code in that issue.

When reporting a vulnerability, include as much of the following as you can:

- The affected component and version.
- The operating system and relevant environment details.
- Steps to reproduce the issue in an environment you own or are authorized to test.
- The potential security impact.
- A minimal proof of concept, if needed to demonstrate the issue.
- Any known workarounds or mitigations.

Do not include passwords, access tokens, private keys, personal information, or data belonging to other people.

We aim to acknowledge reports within 7 calendar days and provide an initial assessment within 14 calendar days. These are response goals, not guaranteed service levels.

Please coordinate public disclosure with the maintainers. We will work with reporters on an appropriate disclosure timeline.

There is currently no formal bug bounty program, and no financial reward is guaranteed.

## Scope

This policy covers security issues in:

- The Veyra interpreter and its built-in modules.
- User-module and VPM package discovery and loading.
- The Python implementation of VPM.
- The repository's official source code and release artifacts.

Examples of potentially in-scope issues include:

- Path traversal or symbolic-link handling that allows package operations or module loading to escape their intended directories.
- Unexpected file overwrite, deletion, or disclosure caused by Veyra or VPM outside their documented behavior.
- A vulnerability that compromises the host process or its data beyond the permissions explicitly granted to the running program.
- Security-sensitive information exposed by project code or official releases.

## Security Boundaries and Limitations

Veyra is **not a sandbox**. A Veyra program runs with the permissions of the user who launched it. Depending on the modules it uses, a program may access files, environment variables, processes, or network resources.

Importing a `.veymod` file executes its module initialization code. Only run Veyra programs and import packages from sources you trust.

VPM 0.1.0 supports local path packages. It does not currently provide remote package installation, lockfiles, cryptographic signatures, or checksum verification. Its path validation, staging, and rollback safeguards do not make an untrusted package safe to install or execute, and they do not provide complete crash-safe transaction guarantees.

Veyra's `random` module provides pseudorandom values and must not be used for cryptographic or security-sensitive purposes.

Expected behavior—such as a Veyra program using documented filesystem, process, or network operations with the permissions of its user—is not by itself a vulnerability. Please report cases where behavior exceeds those documented permissions or affects data outside the intended scope.

## Research Guidelines

We appreciate good-faith security research. Please:

- Test only on systems and data you own or are explicitly authorized to test.
- Avoid accessing, modifying, or disclosing other people's data.
- Do not perform denial-of-service, social-engineering, or destructive testing.
- Stop testing and report privately if you encounter data or access outside your own environment.
- Do not publicly disclose an exploitable vulnerability before coordinating with the maintainers.

This policy does not authorize testing third-party systems or services.
