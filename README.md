[![Tests](https://github.com/omni001s/front_test/actions/workflows/Basic.yml/badge.svg)](https://github.com/omni001s/front_test/actions/workflows/Basic.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![CosmWasm](https://img.shields.io/badge/CosmWasm-green)
![Xion](https://img.shields.io/badge/Xion-black)
# HayPay

# Table of Contents
- [HayPay](#haypay)
- [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Architecture Diagram](#architecture-diagram)
  - [Project Flow](#project-flow)
    - [Token Sender](#token-sender)
    - [Token Claimer](#token-claimer)
  - [Project Highlights](#project-highlights)
      - [HeyPay Contract](#heypay-contract)
  - [Technologies Used](#technologies-used)
  - [Deployed Contract](#deployed-contract)
  - [Conclusion](#conclusion)
  - [What's Next for HeyPay](#whats-next-for-heypay)
  - [Online Demos](#online-demos)
  - [Team](#team)

***
## Overview

HeyPay is an application designed to streamline the user experience for cryptocurrency transfers. By leveraging the Xion blockchain's capabilities, HeyPay allows users to securely transfer tokens using familiar authentication technologies and eliminating the complexities associated with traditional crypto transactions.
***
## Architecture Diagram

<!-- ![image](https://github.com/XXXXX) -->

***
## Project Flow

Here `Bob` is token sender and `Alice` is token claimer.
### Token Sender
1. **Authentication:** `Bob` logs in to HeyPay through Xion dashboard. All users Should use their email address to sign in to HeyPay.

2. **Approve Grant:** `Bob` is redirected to Xion dashboard where he can create a new account or use one of his previously created accounts from list and then `grant` HeyPay to act on his behalf.

3. **Token Transfer:** `Bob` is redirected back to HeyPay web app where he can choose a token from list of tokens and send some amount to an email, he can provide a short memo with his token transfer which will be sent to token `claimer`.

### Token Claimer
1. **Check for Claims:** All users could check if they have tokens to claim, they only need to enter their email address so that HeyPay could check if they have tokens ready to be claimed or not. `Alice` inputs her email address to check for any claimable tokens and she finds that she had some tokens ready to be claimed.

2. **Authentication:** `Alice` uses her email address (which she used to check for any claimable tokens) to sihn in to HeyPay and Xion dashboard where she can choose an account or create a new account just like `Bob`, then she choose one her accounts and give a grant to HeyPay to claim the tokens for her.

3. **Claiming Token:** Upon redirecting to HeyPay, the HeyPay prepares the required transaction to send to blockchain for claiming tokens for `Alice`, HeyPay contract checks the validity of provided security tokens and if the token is valid, it sends the tokens to `Alice`.

***
## Project Highlights

- **Account Abstraction:** The complexities of maintaining key pairs, installing wallets to keep tokens are done using Xion's Account Abstraction and Meta Accounts.

- **Seamless User Experience:** Transferring tokens using HeyPay is as easy that it can be, you only need an email and some tokens in your address to send and receive tokens, no private key, no transaction signing, a strightforward web2 experience in web3!

#### HeyPay Contract

The HeyPay contract, does all the token transferring and token claiming, in this stage, it uses a signed jwt with custom claims to check the authorization of token `claimers`, in future, this contract will accept other forms of users validations for claiming tokens. 

***
## Technologies Used

- **Xion Abstraxion**
- **ReactJs/Next.Js**
- **Rust**
***
## Deployed Contract

The contract is deployed to Xion Testnet at `xionXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`.
You could find more informations about the contract and how to locally test the contract [here](https://github.com/omni001s/HeyPay/tree/contract).


***
## Conclusion

HeyPay tries to bring a seamless user experience for users which do not know blockchain and cryptocurrency.
***
## What's Next for HeyPay

HayPay is an evolving project with a bright future. We envision expanding support to other blockchains by integrating Ibc and adding features like crypto gifting and on-demand swapping. Community engagement is paramount, and we aim to build a vibrant user base through outreach. Ultimately, HayPay strives to be the gateway to a more inclusive and accessible crypto ecosystem.

<!-- send notification to receiver that they have some tokens that are able to be claimed -->

## Online Demos
- [demo video](https://youtu.be/xxxxxxxxxxx)

<!-- - [online demo](https://omni001s.github.io/HeyPay/) -->


***
## Team
- [Meisam](https://www.github.com/meisamtaher)
- [Saeed](https://www.github.com/omni001s)

