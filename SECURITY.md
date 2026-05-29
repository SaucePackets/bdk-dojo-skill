# Security Policy

## Educational only

BDK Dojo is educational. It does not build a production wallet and must not handle real keys, seed phrases, funds, signing, broadcasting, or production wallet safety decisions.

The reference crate teaches toy wallet models only. Real wallet work belongs in carefully reviewed regtest/signet setups and eventually official BDK examples/docs.

## Reporting issues

Report installer or security-sensitive issues privately to the repository owner instead of opening a public issue with exploit details.

## Installer note

The Hermes installer uses Hermes' GitHub skill-directory installer for explicit SaucePackets skill directories:

- `SaucePackets/teach-mode-skill/skills/teach-mode`
- `SaucePackets/bdk-dojo-skill/skills/bdk-dojo`

This installs the full skill directories, including support files like `references/`, into the user's Hermes skills directory.

## Do not commit

- private learner data
- seeds or private keys
- wallet descriptors tied to real funds
- API tokens or credentials
- production wallet configuration
