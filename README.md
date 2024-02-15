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
  - [Project Breakdown:](#project-breakdown)
    - [HeyPay Contract (Rust, CosmWasm, Xion-tailored):](#heypay-contract-rust-cosmwasm-xion-tailored)
    - [Account Dashboard (Next.js, Xion Account Abstraction):](#account-dashboard-nextjs-xion-account-abstraction)
    - [HeyPay Web App (Vite React):](#heypay-web-app-vite-react)
  - [Project Flow](#project-flow)
    - [Token Sender (`Bob`)](#token-sender-bob)
    - [Token Claimer (`Alice`)](#token-claimer-alice)
  - [Project Highlights](#project-highlights)
      - [HeyPay Contract](#heypay-contract)
  - [Technologies Stack:](#technologies-stack)
  - [Deployed Contract](#deployed-contract)
  - [Conclusion](#conclusion)
  - [Envisioning the Future](#envisioning-the-future)
  - [Online Demos](#online-demos)
  - [Team](#team)

***
## Overview

Propelled by Xion's blockchain architecture, HeyPay simplifies cryptocurrency transfers, offering a seamless user experience that eliminates the complexities of conventional crypto transactions. Users leverage familiar authentication methods for secure token transfers, enabling a frictionless web2 experience within the web3 domain.
***
## Architecture Diagram
![Image](https://github.com/omni001s/HeyPay/msg6376040916-569563.jpg)
## Project Breakdown:

### HeyPay Contract (Rust, CosmWasm, Xion-tailored):
The contract is written in rust (cosmwasm) and it manages the token transfer and token claim.
### Account Dashboard (Next.js, Xion Account Abstraction):
A customized verion of Abstraxion dashboard that can create Email JWT alongside wallet connection
### HeyPay Web App (Vite React):
The Front end of our dapp that let users to send and claim token with interacting with Abstraxion dashboard and SmartContract
***
## Project Flow

### Token Sender (`Bob`)
1. **Effortless Login:** `Bob` seamlessly logs in with his email address, eliminating the need for managing intricate private keys.

2. **Granting Permission:** `Bob` authorizes HeyPay to act on his behalf on the Xion blockchain through a straightforward `grant` process.

3. **Intuitive Transfer:** `Bob` effortlessly selects the desired token, specifies the recipient's email address, and defines the transfer amount. An optional memo allows for personalized messages.
4. **Automated Transaction:** HeyPay constructs a transaction and transmits it to the HeyPay contract deployed on the Xion blockchain, handling the technical intricacies behind the scenes.

### Token Claimer (`Alice`)
1. **Claim Inquiry:** `Alice` simply enters her email address to discover any claimable tokens, streamlining the claiming process.

2. **Secure Authentication:** If tokens await claiming, `Alice` utilizes her email address for secure authentication, adhering to privacy and security best practices.

3. **Effortless Claiming:** HeyPay automatically generates a transaction and sends it to the HeyPay contract on the Xion blockchain.
4. **Token Delivery:** Upon verification of `Alice`'s eligibility, the HeyPay contract seamlessly transfers the tokens to her Xion Meta Account, ensuring a smooth and secure delivery.

***
## Project Highlights

- **Account Abstraction:** By leveraging Xion's Account Abstraction and Meta Accounts, HeyPay eliminates the complexities of managing private keys and wallets, significantly enhancing user experience.

- **Seamless User Experience:** HeyPay transforms crypto transfers into a familiar and intuitive process, resembling web2 interactions. Users can send and receive tokens using email addresses, removing the barriers associated with traditional crypto transactions.
- **Empowering HeyPay Contract:** The intelligent HeyPay contract handles all token transfers and claims securely. It utilizes signed JWTs with custom claims for authorization in the current stage, with plans to incorporate diverse validation methods in the future.

#### HeyPay Contract

The HeyPay contract, does all the token transferring and token claiming, in this stage, it uses a signed jwt with custom claims to check the authorization of token `claimers`, in future, this contract will accept other forms of users validations for claiming tokens. 

***
## Technologies Stack:

- **Xion Abstraxion:** Underpins the foundation of secure and efficient blockchain interactions.
- **ReactJs/Next.Js:** Provides the robust framework for building the user-friendly web application.
- **Rust:** Powers the development of the secure and reliable HeyPay contract.
***
## Deployed Contract

The contract is deployed to Xion Testnet at `xion1zt64jdruawm4dl0svxe973gkd4kgvllwzgw6wexlgkpjmcssu4gqnqwdyp`.

**Contract Documentation**: Delve deeper into the contract's intricacies and local testing procedures [here](https://github.com/omni001s/HeyPay/tree/contract)


***
## Conclusion

HeyPay tries to bring a seamless user experience for users which do not know blockchain and cryptocurrency.
***
## Envisioning the Future

HeyPay embarks on an exciting journey of continuous evolution. Expanding support to other blockchains via Ibc integration, incorporating features like crypto gifting and on-demand swapping, and fostering a vibrant community through active outreach are just some of the thrilling milestones on the horizon. Ultimately, HeyPay aspires to be the gateway to a more inclusive and accessible crypto ecosystem, empowering users with a user-centric approach.

<!-- send notification to receiver that they have some tokens that are able to be claimed -->

## Online Demos
- [online demo](https://omni001s.github.io/HeyPay/)
- [demo video](https://youtu.be/xxxxxxxxxxx)



***
## Team
- [Meisam](https://www.github.com/meisamtaher)
- [Saeed](https://www.github.com/omni001s)

