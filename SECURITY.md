# Security Policy

## Educational only

BDK Dojo is educational. It does not build a production wallet and must not handle real keys, seed phrases, funds, signing, broadcasting, or production wallet safety decisions.

The reference crate teaches toy wallet models only. Real wallet work belongs in carefully reviewed regtest/signet setups and eventually official BDK examples/docs.

## Reporting issues

Report installer or security-sensitive issues privately to the repository owner instead of opening a public issue with exploit details.

## Installer note

The Hermes installer uses `--force` only for explicit SaucePackets skill URLs:

- `SaucePackets/teach-mode-skill`
- `SaucePackets/bdk-dojo-skill`

This works around a current false-positive Hermes community-skill scanner issue on Teach Mode. The installer then verifies that both expected skills are visible after install.

## Do not commit

- private learner data
- seeds or private keys
- wallet descriptors tied to real funds
- API tokens or credentials
- production wallet configuration
