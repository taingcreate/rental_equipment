# Project Title

Rental Equipment – A Soroban Smart Contract for Equipment Rentals on Stellar

## Project Vision

This project demonstrates a ** decentralized equipment rental system** built on Soroban and Stellar. It provides:
- How to write a Soroban smart contract in Rust for real-world use cases
- How to manage persistent storage for equipment registration and rental tracking
- How to handle user authentication and ownership in smart contracts
- How to deploy and interact with contracts on Stellar Testnet

The goal is to provide a working example of an equipment rental deposit system that can be studied, deployed, and extended.

---

## Description

A Soroban smart contract dApp that enables equipment owners to register their equipment and renters to rent it with daily rates on Stellar Testnet. The contract handles rental registration, cost calculation, and payment claims in a decentralized manner.

---

## Features

### 1. Equipment Registration
- Owners can register equipment with a unique ID and daily rental rate
- On-chain tracking of equipment ownership and availability
- Authenticated registration requiring owner authorization

### 2. Equipment Rental
- Renters can rent available equipment for a specified number of days
- Automatic cost calculation (daily_rate * days)
- Tracks rental period with return block number

### 3. Payment Claims
- Owners can claim payment after the rental period ends
- Secure release of equipment back to available state after claim

### 4. On-chain Transparency
- All equipment data stored permanently on blockchain
- Anyone can verify equipment status, ownership, and rental history

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CCVDQYZII7JP3MKROFW55S7TJQVTLYWTAJBLIVFKKKSSOIHC6MXVJ3KC](https://stellar.expert/explorer/testnet/tx/a556b71303c3701554bbaca127ae14f9af02b9ca4a7cf3e5f5925772467e3c7d)

![screenshot](https://i.ibb.co/202zJ6Yz/image.png)

---

## Future Scopes

### 1. Payment Integration
- Integrate with Stellar native tokens (XLM) for actual payment transfers
- Escrow functionality to hold and release funds securely

### 2. Late Return Penalties
- Add penalty fees for equipment returned after the specified return block
- Automatic penalty calculation and enforcement

### 3. Multiple Equipment Types
- Support categorized equipment with different metadata
- Search and filter equipment by type, price range, availability

### 4. Frontend dApp
- Build a simple web interface using React or plain HTML/JS for easier user interaction
- Equipment listing and rental dashboard

### 5. Rating System
- Add post-rental rating mechanism for both owners and renters
- On-chain reputation tracking

### 6. Insurance Deposits
- Implement refundable insurance deposits for high-value equipment
- Automatic deposit management based on equipment value

---

## Profile

- **Name:** taingcreate

