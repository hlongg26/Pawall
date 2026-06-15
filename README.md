<img width="1918" height="873" alt="image" src="https://github.com/user-attachments/assets/cd0a2209-6a59-47ae-a89b-a08b45909bc6" />
# Title
Stellar Paywall: Pay-to-Unlock Content

# Description
I built this project to solve the problem of monetizing small digital goods or premium content. Traditional payment methods are inefficient for micro-transactions due to high fees. By leveraging the Stellar network and Soroban smart contracts, this project allows creators to charge users a very small fee (e.g., 0.1 XLM) to unlock hidden content, taking full advantage of Stellar's high speed and near-zero transaction costs.

# Features
- **One-Click Micropayments:** Users can pay a predefined amount of XLM directly from their wallet to the admin's wallet.
- **Access Verification:** A read-only `check_access` function allows the frontend to instantly verify if a user has unlocked the content without incurring network fees.
- **Decentralized Ownership:** Payment records are permanently stored on-chain, ensuring transparency.
- **Simple On-Chain Interaction:** Designed with simplicity in mind, executing only one primary state-changing transaction (`unlock`).

# Contract
Contract link: https://stellar.expert/explorer/testnet/tx/709c90b7c80b4c505c31871e7ab4dffb246ea7e54421a41dd12e228274ea84f4

*(Lưu ý cho bạn: Nhớ chèn/upload thêm 1 tấm ảnh chụp màn hình Soroban Studio lúc deploy thành công có chữ SUCCESS xanh lá cây vào bên dưới dòng này nhé)*

# Future scopes
My future plan is to integrate this smart contract directly into my personal web project, "Blog Hoàng Long", allowing me to token-gate premium technical articles. Furthermore, as I am highly interested in Artificial Intelligence, I plan to explore integrating AI models to auto-generate or summarize the premium content being unlocked on the platform.

# Profile
Hoang Long - Computer Science student at HUTECH. I have a strong foundation in C/C++ programming (Data Structures, Algorithms) and a deep passion for becoming an AI Engineer. I am currently expanding my skill set into blockchain development and Web3 integrations using Stellar and Soroban.
