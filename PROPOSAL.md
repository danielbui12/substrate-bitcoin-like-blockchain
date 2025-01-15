Phase 1: Research & Preparation

1. Understand Requirements
   - Understanding Bitcoin’s architecture, including UTXO models and address formats (Legacy, SegWit, and Taproot).
   - Study Substrate’s modular framework and PoW consensus capabilities.  

2. Set Up Environment  
   - Install Substrate Node Template as a starting point.  
   - Configure development environment (Rust toolchain, Substrate dependencies, etc.).  

---

Phase 2: Implementation

1. Implement Core Blockchain Features  
   - Implement PoW node
      - Mining and block production with multiple hashing algorithm md5, sha3, keccak.
   - Develop pallets/node for:  
     - Block author & difficulty, issuance (total supply, Bitcoin halving).
     - UTXO account model.

2. Testing & Optimization
   - Test the implementation.
   - Comment detailed on function, storage, etc
   - Clean code.

---

Phase 3: Documentation

1. Write Development Guidelines
   - About Proof-of-Work:
      - What is PoW.
      - How PoW work.
      - Comparing with Proof-of-Stake.
      - Code breakdown for PoW.
   - UTXO account model:
      - What is UTXO account model.
      - Comparing with Account based.
   - Document project setup and environment configuration:
      - Comment detailed on code logic.
      - Provide reference link to relevant Polkadot SDK implementation & knowledge
      - Code breakdown for UTXO.
   - Provide step-by-step guides for:  
      - Running the blockchain locally.
      - Demo usage of UTXO account model.

2. User Documentation
   - Include FAQs and troubleshooting steps.  

Phase 4: Update following feedback

Updating following feedback of OpenGuild
